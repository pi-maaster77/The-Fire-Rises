// devtools/src/types/Map.ts

export interface MapData {
  map_params: {
    width: number
    height: number
    voronoi_points: number
  }
  seed_points: [number, number][]
  provinces: Array<{
    id: string
    seed_index: number
    center: [number, number]
  }>
}
