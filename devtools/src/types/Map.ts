// devtools/src/types/Map.ts

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
