/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/engine/MapRender.ts *
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

import { Container, Graphics } from 'pixi.js'
import { Voronoi, Delaunay } from 'd3-delaunay'
import type { MapData } from '@/types/Map'

export class MapRenderer {
  private backgroundLayer = new Container()
  private editLayer = new Container()
  private highlightGraphics = new Graphics()

  // Guardamos un mapa de los gráficos de edición indexados por ID de provincia
  private activeEdits = new Map<number, Graphics>()

  constructor(rootContainer: Container) {
    // Definimos el orden estricto de profundidad (Z-indexing)
    this.backgroundLayer.eventMode = 'none'
    this.backgroundLayer.interactiveChildren = false

    rootContainer.addChild(this.backgroundLayer)
    rootContainer.addChild(this.editLayer)
    rootContainer.addChild(this.highlightGraphics)
  }

  public renderBaseMap(data: MapData, voronoi: Voronoi<Delaunay.Point[]>) {
    this.clear()
    const mainGraphics = new Graphics()

    data.provinces.forEach((province) => {
      const polygon = voronoi.cellPolygon(province.seed_index)
      if (!polygon) return

      const color = province.state_id ? 0x4444aa : 0x333333
      mainGraphics
        .poly(polygon.flat())
        .fill({ color, alpha: 0.8 })
        .stroke({ width: 0.5, color: 0x000000, alpha: 0.2 })
    })

    this.backgroundLayer.addChild(mainGraphics)
    this.backgroundLayer.cacheAsBitmap = true // Ganancia masiva de FPS en mapas grandes
  }

  public updateProvinceVisual(index: number, voronoi: Voronoi<Delaunay.Point[]>, color: number) {
    const polygon = voronoi.cellPolygon(index)
    if (!polygon) return

    // Si ya existía un override visual para esta provincia, lo limpiamos para no acumular memoria
    if (this.activeEdits.has(index)) {
      this.activeEdits.get(index)!.destroy()
    }

    const graphic = new Graphics()
    graphic
      .poly(polygon.flat())
      .fill({ color, alpha: 0.9 })
      .stroke({ width: 1, color: 0xffffff, alpha: 0.5 })

    this.editLayer.addChild(graphic)
    this.activeEdits.set(index, graphic)
  }

  public highlightProvince(index: number, voronoi: Voronoi<Delaunay.Point[]>) {
    const polygon = voronoi.cellPolygon(index)
    this.highlightGraphics.clear()

    if (polygon) {
      this.highlightGraphics
        .poly(polygon.flat())
        .fill({ color: 0xccff00, alpha: 0.2 })
        .stroke({ width: 3, color: 0xccff00, alpha: 1 })
    }
  }

  public clear() {
    this.backgroundLayer.removeChildren()
    this.backgroundLayer.cacheAsBitmap = false
    this.editLayer.removeChildren()
    this.highlightGraphics.clear()
    this.activeEdits.clear()
  }
}
