// devtools/src/engine/MapEngine.ts

import { Application, Container, Graphics } from 'pixi.js'
import { Viewport } from 'pixi-viewport'
import { Delaunay } from 'd3-delaunay'
import type { MapData } from '@/types/Map'

export class MapEngine {
  public app: Application
  public viewport: Viewport
  public world: Container
  private provinceLayer: Container

  constructor(parentElement: HTMLElement) {
    this.app = new Application()
    this.provinceLayer = new Container()
    this.init(parentElement)
  }

  private async init(parentElement: HTMLElement) {
    await this.app.init({
      resizeTo: parentElement,
      backgroundColor: 0x1a1a2e,
      antialias: true,
    })

    parentElement.appendChild(this.app.canvas)

    this.viewport = new Viewport({
      screenWidth: window.innerWidth,
      screenHeight: window.innerHeight,
      worldWidth: 2000,
      worldHeight: 2000,
      events: this.app.renderer.events,
    })

    this.viewport.drag().pinch().wheel().decelerate()
    this.app.stage.addChild(this.viewport)

    this.world = new Container()
    this.viewport.addChild(this.world)
    this.world.addChild(this.provinceLayer)
  }

  /**
   * Procesa el JSON y dibuja las provincias
   */
  public renderMap(data: MapData) {
    // 1. Limpiar capa previa
    this.provinceLayer.removeChildren()

    // 2. Configurar el tamaño del mundo según el JSON
    const { width, height } = data.map_params
    this.viewport.worldWidth = width
    this.viewport.worldHeight = height

    // 3. Crear el Voronoi
    // d3-delaunay necesita un array plano o de pares para los puntos
    const delaunay = Delaunay.from(data.seed_points)
    const voronoi = delaunay.voronoi([0, 0, width, height])

    // 4. Dibujar cada provincia
    data.provinces.forEach((province) => {
      const polygon = voronoi.cellPolygon(province.seed_index)

      if (polygon) {
        const graphics = new Graphics()

        // Color aleatorio para diferenciar provincias (puedes usar lógica de regiones luego)
        const randomColor = Math.floor(Math.random() * 0xffffff)

        // Estilo v8
        graphics.context
          .beginPath()
          .poly(polygon.flat(), true) // .flat() convierte [[x,y], [x,y]] en [x,y,x,y]
          .fill({ color: randomColor, alpha: 0.6 })
          .stroke({ width: 2, color: 0xffffff, alpha: 0.3 })

        // Hacer la provincia interactiva
        graphics.eventMode = 'static'
        graphics.cursor = 'pointer'
        graphics.on('pointerover', () => {
          graphics.alpha = 1.5 // Efecto de brillo
        })
        graphics.on('pointerout', () => {
          graphics.alpha = 1
        })

        this.provinceLayer.addChild(graphics)
      }
    })
  }
}
