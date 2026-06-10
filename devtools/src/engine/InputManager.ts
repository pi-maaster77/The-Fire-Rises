/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/engine/InputManager.ts *
 * Project: The Fire Rises                                                     *
 * Created Date: We Jun 2026                                                   *
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
 * Date      	By	Comments                                                   *
 * ----------	---	---------------------------------------------------------  *
 */

import { Point } from 'pixi.js'
import { Viewport } from 'pixi-viewport'
import { Delaunay } from 'd3-delaunay'

export class InputManager {
  private viewport: Viewport
  private delaunayProvider: () => Delaunay<Delaunay.Point[]> | null
  private isMouseDown = false
  private lastHoverIndex: number | null = null

  public onBrushStroke?: (index: number) => void
  public onClick?: (index: number) => void

  constructor(viewport: Viewport, delaunayProvider: () => Delaunay<Delaunay.Point[]> | null) {
    this.viewport = viewport
    this.delaunayProvider = delaunayProvider
    this.initListeners()
  }

  private initListeners() {
    this.viewport.on('pointerdown', (e) => {
      this.isMouseDown = true
      this.processInput(e.global, 'down')
    })

    this.viewport.on('pointermove', (e) => {
      this.processInput(e.global, 'move')
    })

    window.addEventListener('pointerup', () => {
      this.isMouseDown = false
      this.lastHoverIndex = null
    })
  }

  // path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/engine/InputManager.ts

  private processInput(globalPos: Point, type: 'down' | 'move') {
    const delaunay = this.delaunayProvider()
    if (!delaunay) return

    const worldPos = this.viewport.toWorld(globalPos)
    const index = delaunay.find(worldPos.x, worldPos.y)

    if (index === -1) return

    // 1. Si es un Click inicial (pointerdown), siempre disparamos el onClick primero
    if (type === 'down' && this.onClick) {
      this.onClick(index)
    }

    // 2. Si el mouse está apretado y hay un callback de brocha activo, manejamos el arrastre
    if (this.isMouseDown && this.onBrushStroke) {
      if (index !== this.lastHoverIndex) {
        this.lastHoverIndex = index
        this.onBrushStroke(index)
      }
    }
  }
}
