// devtools/src/engine/MapEngine.ts
import { Application, Container, Graphics, Point } from 'pixi.js'
import { Viewport } from 'pixi-viewport'
import { Delaunay, Voronoi } from 'd3-delaunay'
import type { MapData } from '@/types/Map'

export class MapEngine {
  public app: Application
  public viewport!: Viewport
  public initPromise: Promise<void>

  private mapData: MapData | null = null
  private voronoi: Voronoi<Delaunay.Point[]> | null = null
  private delaunay: Delaunay<Delaunay.Point[]> | null = null

  // Capas con orden Z definido
  private backgroundLayer = new Container()
  private editLayer = new Container()
  private highlightGraphics = new Graphics()

  // Estado del Input
  private isMouseDown = false
  private lastHoverIndex: number | null = null

  // Callbacks para Vue
  public onBrushStroke?: (provinceIndex: number) => void
  public onProvinceClick?: (provinceIndex: number) => void

  constructor(parentElement: HTMLElement) {
    this.app = new Application()
    this.initPromise = this.init(parentElement)
  }

  private async init(parentElement: HTMLElement) {
    await this.app.init({
      resizeTo: parentElement,
      backgroundColor: 0x0a0a12,
      antialias: false,
      resolution: window.devicePixelRatio || 1,
    })

    parentElement.appendChild(this.app.canvas)

    // CONFIGURACIÓN ÚNICA DEL VIEWPORT
    this.viewport = new Viewport({
      screenWidth: window.innerWidth,
      screenHeight: window.innerHeight,
      events: this.app.renderer.events,
      // disableOnCheck: true,
    })

    this.viewport.drag().pinch().wheel().decelerate()
    this.app.stage.addChild(this.viewport)

    // Configuración de capas
    this.backgroundLayer.eventMode = 'none'
    this.backgroundLayer.interactiveChildren = false

    // Añadir en orden de profundidad
    this.viewport.addChild(this.backgroundLayer)
    this.viewport.addChild(this.editLayer)
    this.viewport.addChild(this.highlightGraphics)

    // LISTENERS UNIFICADOS
    this.viewport.on('pointerdown', (e) => {
      this.isMouseDown = true
      this.handleInput(e.global, 'down')
    })

    this.viewport.on('pointermove', (e) => {
      this.handleInput(e.global, 'move')
    })

    window.addEventListener('pointerup', () => {
      this.isMouseDown = false
      this.lastHoverIndex = null
    })
  }

  // Control central de interacción
  private handleInput(globalPos: Point, type: 'down' | 'move') {
    if (!this.delaunay || !this.mapData) return

    const worldPos = this.viewport.toWorld(globalPos)
    const index = this.delaunay.find(worldPos.x, worldPos.y)

    if (index === -1) return

    // Lógica de Brocha (si está presionado y hay callback)
    if (this.isMouseDown && this.onBrushStroke) {
      if (index !== this.lastHoverIndex) {
        this.lastHoverIndex = index
        this.onBrushStroke(index)
      }
    }

    // Lógica de Resaltado/Click (si solo es movimiento o click puntual)
    if (type === 'down' && !this.onBrushStroke) {
      this.handleMapClick(index)
    }
  }

  public setCameraControl(enabled: boolean) {
    if (enabled) {
      this.viewport.plugins.resume('drag')
      this.app.canvas.style.cursor = 'grab'
    } else {
      this.viewport.plugins.pause('drag')
      this.app.canvas.style.cursor = 'crosshair'
    }
  }

  public renderMap(data: MapData) {
    this.mapData = data
    this.backgroundLayer.removeChildren()
    this.editLayer.removeChildren() // Limpiar edición al cargar nuevo mapa

    const { width, height } = data.map_params
    this.delaunay = Delaunay.from(data.seed_points)
    this.voronoi = this.delaunay.voronoi([0, 0, width, height])

    const mainGraphics = new Graphics()

    data.provinces.forEach((province) => {
      const polygon = this.voronoi!.cellPolygon(province.seed_index)
      if (polygon) {
        const color = province.state_id ? 0x4444aa : 0x333333
        mainGraphics
          .poly(polygon.flat())
          .fill({ color, alpha: 0.8 })
          .stroke({ width: 0.5, color: 0x000000, alpha: 0.2 })
      }
    })

    this.backgroundLayer.addChild(mainGraphics)
    this.backgroundLayer.cacheAsBitmap = true
    this.viewport.fitWorld()
  }

  public updateProvinceVisual(index: number, color: number) {
    const polygon = this.voronoi?.cellPolygon(index)
    if (polygon) {
      const p = new Graphics()
      p.poly(polygon.flat())
        .fill({ color, alpha: 0.9 })
        .stroke({ width: 1, color: 0xffffff, alpha: 0.5 })
      this.editLayer.addChild(p)
    }
  }

  private handleMapClick(index: number) {
    const province = this.mapData?.provinces[index]
    if (province) {
      const polygon = this.voronoi!.cellPolygon(index)
      if (polygon) {
        this.highlightGraphics.clear()
        this.highlightGraphics
          .poly(polygon.flat())
          .fill({ color: 0xccff00, alpha: 0.2 })
          .stroke({ width: 3, color: 0xccff00, alpha: 1 })
      }
      if (this.onProvinceClick) this.onProvinceClick(index)
    }
  }
}
