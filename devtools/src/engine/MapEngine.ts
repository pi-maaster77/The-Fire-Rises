/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/engine/MapEngine.ts *
 * Project: The Fire Rises                                                     *
 * Created Date: Mo Jun 2026                                                   *
 * Author: pi-maaster77 (pimaaster1337@gmail.com)                              *
 * -----                                                                       *
 * Last Modified: Wed Jun 10 2026                                              *
 * Modified By: pi-maaster77                                                   *
 * -----                                                                       *
 * Copyright (c) projectCreationYear-2026 The Fire Rises                       *
 * -----                                                                       *
 * License: GNU Affero General Public License v3.0 (AGPL-3.0)                  *
 * This program is free software: you can redistribute it and/or modify        *
 * it under the terms of the GNU Affero General Public License as              *
 * published by the Free Software Foundation, either version 3 of the          *
 * License, or (at your option) any later version.                             *
 * -----                                                                       *
 * HISTORY:                                                                    *
 * Date      	By	Comments                                                     *
 * ----------	---	---------------------------------------------------------    *
 */

import { Application } from 'pixi.js'
import { Viewport } from 'pixi-viewport'
import { Delaunay, Voronoi } from 'd3-delaunay'
import { InputManager } from './InputManager'
import { MapRenderer } from './MapRenderer'
import type { MapData } from '@/types/Map'

export class MapEngine {
  public app: Application
  public viewport!: Viewport
  public initPromise: Promise<void>

  private renderer!: MapRenderer
  private input!: InputManager

  private mapData: MapData | null = null
  private voronoi: Voronoi<Delaunay.Point[]> | null = null
  private delaunay: Delaunay<Delaunay.Point[]> | null = null

  // Callbacks limpios para la interfaz de Vue
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

    this.viewport = new Viewport({
      screenWidth: window.innerWidth,
      screenHeight: window.innerHeight,
      events: this.app.renderer.events,
    })

    this.viewport.drag().pinch().wheel().decelerate()
    this.app.stage.addChild(this.viewport)

    // Inicializar módulos secundarios
    this.renderer = new MapRenderer(this.viewport)
    this.input = new InputManager(this.viewport, () => this.delaunay)

    // Vincular inputs del gestor con la lógica del motor
    this.input.onBrushStroke = (index) => this.handleBrush(index)
    this.input.onClick = (index) => this.handleClick(index)
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
    const { width, height } = data.map_params

    this.delaunay = Delaunay.from(data.seed_points)
    this.voronoi = this.delaunay.voronoi([0, 0, width, height])

    this.renderer.renderBaseMap(data, this.voronoi)
    this.viewport.fitWorld()
  }

  public updateProvinceVisual(index: number, color: number) {
    if (!this.voronoi) return
    this.renderer.updateProvinceVisual(index, this.voronoi, color)
  }

  private handleBrush(index: number) {
    if (this.onBrushStroke) {
      this.onBrushStroke(index)
    }
  }

  private handleClick(index: number) {
    if (!this.mapData || !this.voronoi) return

    const province = this.mapData.provinces[index]
    if (province) {
      this.renderer.highlightProvince(index, this.voronoi)
      if (this.onProvinceClick) {
        this.onProvinceClick(index)
      }
    }
  }
}
