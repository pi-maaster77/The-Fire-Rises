// devtools/src/engine/MapEngine.ts

import { Application, Container, Graphics, Color } from 'pixi.js';
import { Viewport } from 'pixi-viewport';

export class MapEngine {
    public app: Application;
    public viewport: Viewport;
    public world: Container;

    constructor(parentElement: HTMLElement) {
        // Inicializar App
        this.app = new Application();
        
        // Usamos una función asíncrona interna para inicializar Pixi v8
        this.init(parentElement);
    }

    private async init(parentElement: HTMLElement) {
        await this.app.init({ 
            resizeTo: parentElement,
            backgroundColor: 0x1a1a2e, // Un azul oscuro "geopolítico"
            antialias: true 
        });
        
        parentElement.appendChild(this.app.canvas);

        // Crear Cámara (Viewport)
        this.viewport = new Viewport({
            screenWidth: window.innerWidth,
            screenHeight: window.innerHeight,
            worldWidth: 10000, // Tu mapa es grande
            worldHeight: 10000,
            events: this.app.renderer.events,
        });

        // Configurar controles de cámara
        this.viewport
            .drag()
            .pinch()
            .wheel()
            .decelerate();

        this.app.stage.addChild(this.viewport);

        // Contenedor principal donde "vive" el mapa
        this.world = new Container();
        this.viewport.addChild(this.world);

        // Dibujamos un "Dummy" para probar la cámara
        this.createTestMap();
    }

    private createTestMap() {
        const grid = new Graphics();
        // Dibujamos una grilla de referencia
        grid.setStrokeStyle({ width: 2, color: 0x333355 });
        for (let i = 0; i < 10000; i += 500) {
            grid.moveTo(i, 0).lineTo(i, 10000);
            grid.moveTo(0, i).lineTo(10000, i);
        }
        grid.stroke();
        this.world.addChild(grid);
    }
}