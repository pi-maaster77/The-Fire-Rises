/*
 * Path: /mnt/trd/Proyectos/rust/The Fire Rises/devtools/src/types/Map.ts      *
 * Project: The Fire Rises                                                     *
 * Created Date: Mo Jun 2026                                                   *
 * Author: pi-maaster77 (pimaaster1337@gmail.com)                              *
 * -----                                                                       *
 * Last Modified: Mon Jun 08 2026                                              *
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

export interface Province {
  id: string
  seed_index: number
  center: [number, number]
  state_id: string | null
}

export interface State {
  id: string
  name: string
  provinces: number[]
  color: string
}

export interface MapData {
  map_params: {
    width: number
    height: number
    voronoi_points: number
  }
  seed_points: [number, number][]
  provinces: Array<Province>
  states: Array<State>
}
