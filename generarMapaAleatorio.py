# generarMapaAleatorio.py

# este archivo genera un mapa de vornoi aleatorio con relajación de Lloyd, lo colorea evitando vecinos iguales y exporta un JSON para el editor

import numpy as np
from scipy.spatial import Voronoi
from PIL import Image, ImageDraw
import random

WIDTH = 1024
HEIGHT = 1024
NUM_POINTS = 20000
RELAX_ITER = 3

COLORS = ["#FFFF00", "#FFFF22", "#FFFF44", "#FFFF66", "#FFFF88", "#FFFFAA", "#FFFFCC", "#FFFFFF"]

# ----------------------------
# Lloyd relaxation
# ----------------------------
def lloyd_relaxation(points, bounds, iterations):
    for _ in range(iterations):
        vor = Voronoi(points)
        new_points = []

        for i, region_index in enumerate(vor.point_region):
            region = vor.regions[region_index]
            if -1 in region or len(region) == 0:
                new_points.append(points[i])
                continue

            polygon = np.array([vor.vertices[v] for v in region])
            centroid = polygon.mean(axis=0)

            # clamp dentro del mapa
            centroid[0] = np.clip(centroid[0], 0, bounds[0])
            centroid[1] = np.clip(centroid[1], 0, bounds[1])

            new_points.append(centroid)

        points = np.array(new_points)

    return points

# ----------------------------
# Generar puntos
# ----------------------------
points = np.column_stack((
    np.random.uniform(0, WIDTH, NUM_POINTS),
    np.random.uniform(0, HEIGHT, NUM_POINTS)
))

points = lloyd_relaxation(points, (WIDTH, HEIGHT), RELAX_ITER)

vor = Voronoi(points)

# ----------------------------
# Crear vecinos (grafo)
# ----------------------------
neighbors = {i: set() for i in range(len(points))}

for (p1, p2) in vor.ridge_points:
    neighbors[p1].add(p2)
    neighbors[p2].add(p1)

order = list(range(len(points)))
order.sort(key=lambda i: len(neighbors[i]), reverse=True)
# ----------------------------
# Colorear evitando vecinos iguales
# ----------------------------
cell_colors = {}

order = list(range(len(points)))
random.shuffle(order)

for i in order:
    score = {c: 0 for c in COLORS}

    # penalizar vecinos directos FUERTE
    for n in neighbors[i]:
        if n in cell_colors:
            score[cell_colors[n]] += 100

    # penalizar vecinos de vecinos
    for n in neighbors[i]:
        for nn in neighbors[n]:
            if nn in cell_colors:
                score[cell_colors[nn]] += 10

    # penalizar vecinos lejanos (rompe manchas grandes)
    for n in neighbors[i]:
        for nn in neighbors[n]:
            for nnn in neighbors[nn]:
                if nnn in cell_colors:
                    score[cell_colors[nnn]] += 1

    # elegir el color con menor penalización (usando score.get como clave)
    best_color = min(score.keys(), key=lambda c: score[c])

    cell_colors[i] = best_color
# ----------------------------
# Función para regiones finitas (simplificada y estable)
# ----------------------------
def get_finite_polygons(vor, width, height):
    regions = []
    for region_index in vor.point_region:
        region = vor.regions[region_index]

        polygon = [vor.vertices[v] for v in region]
        regions.append(polygon)

    return regions

regions = get_finite_polygons(vor, WIDTH, HEIGHT)

# ----------------------------
# Dibujar
# ----------------------------
img = Image.new("RGB", (WIDTH, HEIGHT), "white")
draw = ImageDraw.Draw(img)

for i, region_index in enumerate(vor.point_region):
    region = vor.regions[region_index]

    if -1 in region or len(region) == 0:
        continue

    polygon = [tuple(vor.vertices[v]) for v in region]

    draw.polygon(polygon, fill=cell_colors[i])

img.save("mapa_random.png")
print("OK -> mapa_random.png")

# ----------------------------
# Exportar para el editor
# ----------------------------
import json

# Convertir puntos a lista de listas para JSON
seed_points = [[float(p[0]), float(p[1])] for p in points]

# Crear provincias con seed_index
provinces = []
for i in range(len(points)):
    provinces.append({
        "id": f"{i:08d}",
        "seed_index": i,
        "state_id": None,
        "region_id": None,
        "center": [float(points[i][0]), float(points[i][1])]
    })

export_data = {
    "version": "1.0",
    "map_params": {
        "width": WIDTH,
        "height": HEIGHT,
        "voronoi_points": NUM_POINTS,
        "lloyd_iterations": RELAX_ITER
    },
    "seed_points": seed_points,
    "regions": [],
    "states": [],
    "provinces": provinces
}

# Guardar como JSON minificado para menor tamaño
with open("map_export.json", "w") as f:
    json.dump(export_data, f, separators=(',', ':'), indent=4)

# Calcular tamaño
json_size = len(json.dumps(export_data, separators=(',', ':'), indent=4)) / (1024 * 1024)
print(f"OK -> map_export.json ({json_size:.2f} MB)")
print(f"   - {len(points)} provincias")
print(f"   - {len(seed_points)} seed_points")