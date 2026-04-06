// devtools/src/engine/MapEngine.ts

import { Application, Container, Graphics, Point } from 'pixi.js'
import { Viewport } from 'pixi-viewport'
import { Delaunay, Voronoi } from 'd3-delaunay'
import type { MapData, Province } from '@/types/Map'

export class MapEngine {
  public app: Application
  public viewport!: Viewport
  public initPromise: Promise<void>

  private mapData: MapData | null = null
  private voronoi: Voronoi<Delaunay.Point[]> | null = null
  private delaunay: Delaunay<Delaunay.Point[]> | null = null

  // Capas
  private backgroundLayer = new Container()
  private highlightGraphics = new Graphics() // Solo para la provincia bajo el mouse

  // Brocha
  private editLayer = new Container() // Nueva capa para provincias modificadas
  public onProvinceClick?: (index: number) => void
  private isMouseDown = false
  public onBrushStroke?: (provinceIndex: number) => void
  private lastHoverIndex: number | null = null // Para optimizar la brocha

  constructor(parentElement: HTMLElement) {
    this.app = new Application()
    this.initPromise = this.init(parentElement)
  }

  private async init(parentElement: HTMLElement) {
    await this.app.init({
      resizeTo: parentElement,
      backgroundColor: 0x0a0a12,
      antialias: false, // Desactivar si el rendimiento es crítico a 50k+
      resolution: window.devicePixelRatio || 1,
    })

    parentElement.appendChild(this.app.canvas)

    this.viewport = new Viewport({
      events: this.app.renderer.events,
    })

    this.viewport.drag().pinch().wheel().decelerate()
    this.app.stage.addChild(this.viewport)

    this.viewport.addChild(this.backgroundLayer)
    this.viewport.addChild(this.highlightGraphics)

    // Evento centralizado de mouse
    this.viewport.on('click', (e) => this.handleMapClick(e.global))

    this.app.renderer.events.cursorStyles.default = 'default'

    this.viewport = new Viewport({
      events: this.app.renderer.events,
      disableOnCheck: true, // Optimización v8
    })

    // IMPORTANTE: El fondo no debe procesar eventos individuales
    this.backgroundLayer.eventMode = 'none'
    this.backgroundLayer.interactiveChildren = false

    this.viewport.drag().pinch().wheel().decelerate()
    this.app.stage.addChild(this.viewport)

    this.viewport.addChild(this.backgroundLayer)
    this.viewport.addChild(this.highlightGraphics)

    // Escuchamos el click solo en el viewport
    this.viewport.on('pointertap', (e) => this.handleMapClick(e.global))

    this.viewport.addChild(this.backgroundLayer)
    this.viewport.addChild(this.editLayer) // Capa intermedia
    this.viewport.addChild(this.highlightGraphics)

    // Listeners para la brocha
    this.viewport.on('pointerdown', (e) => {
      this.isMouseDown = true
      this.handleBrush(e.global)
    })

    this.viewport.on('pointermove', (e) => {
      if (this.isMouseDown) {
        this.handleBrush(e.global)
      }
    })

    window.addEventListener('pointerup', () => {
      this.isMouseDown = false
      this.lastHoverIndex = null // Reset para permitir pintar la misma al volver
    })
  }

  public renderMap(data: MapData) {
    this.mapData = data
    this.backgroundLayer.removeChildren()

    const { width, height } = data.map_params
    this.delaunay = Delaunay.from(data.seed_points)
    this.voronoi = this.delaunay.voronoi([0, 0, width, height])

    const mainGraphics = new Graphics()

    // Dibujamos todas las provincias en UN SOLO comando de renderizado
    data.provinces.forEach((province) => {
      const polygon = this.voronoi!.cellPolygon(province.seed_index)
      if (polygon) {
        // Aquí puedes usar un color basado en la región/estado del JSON
        const color = province.state_id ? 0x4444aa : 0x333333

        mainGraphics
          .poly(polygon.flat())
          .fill({ color, alpha: 0.8 })
          .stroke({ width: 0.5, color: 0x000000, alpha: 0.2 })
      }
    })

    this.backgroundLayer.addChild(mainGraphics)

    // Esto convierte 20k polígonos en una sola imagen en memoria de GPU
    this.backgroundLayer.cacheAsBitmap = true

    this.viewport.fitWorld()
  }

  public updateProvinceVisual(index: number, color: number) {
    if (!this.voronoi) return

    const polygon = this.voronoi.cellPolygon(index)
    if (polygon) {
      const pGraphics = new Graphics()
      pGraphics
        .poly(polygon.flat())
        .fill({ color, alpha: 0.9 })
        .stroke({ width: 1, color: 0xffffff, alpha: 0.5 })

      // Añadimos a la capa de edición
      this.editLayer.addChild(pGraphics)
    }
  }

  private handleBrush(globalPos: Point) {
    if (!this.delaunay || !this.mapData) return

    const worldPos = this.viewport.toWorld(globalPos)
    const index = this.delaunay.find(worldPos.x, worldPos.y)

    // Evitamos repintar la misma provincia mil veces mientras el mouse está quieto
    if (index !== -1 && index !== this.lastHoverIndex) {
      this.lastHoverIndex = index

      if (this.onBrushStroke) {
        this.onBrushStroke(index)
      }
    }
  }

  private handleMapClick(globalPos: Point) {
    if (!this.delaunay || !this.voronoi || !this.mapData) return

    // 1. Convertir coordenadas
    const worldPos = this.viewport.toWorld(globalPos)

    // 2. Encontrar el índice (esto es O(log n), extremadamente rápido)
    const index = this.delaunay.find(worldPos.x, worldPos.y)

    if (index !== -1) {
      const province: Province | undefined = this.mapData.provinces[index]
      console.log(`Provincia clickeada: ${province?.id}`, province)

      const polygon = this.voronoi.cellPolygon(index)
      if (polygon) {
        // 3. Dibujar el resaltado solo una vez
        this.highlightGraphics.clear()
        this.highlightGraphics
          .poly(polygon.flat())
          .fill({ color: 0xccff00, alpha: 0.2 })
          .stroke({ width: 3, color: 0xccff00, alpha: 1 })
      }
    }
  }
}
