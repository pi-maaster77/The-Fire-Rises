// devtools/web-ui/src/wasm/bridge.ts
import { useMapStore } from '../stores/map'
import { useRegionStore } from '../stores/region.js';
// devtools/web-ui/src/wasm/bridge.js (o donde tengas tus exportaciones)
import { update_brush_settings } from './map_editor.js';

// Definimos la estructura de los mensajes para tener autocompletado (IntelliSense)
export type InboundEvent = 
  | { type: 'COUNTER_UPDATED'; payload: number }
  | { type: 'PROVINCE_SELECTED'; payload: { province: any; state: any } }
  | { type: 'SCAN_COMPLETED'; payload: { provinces: string[] } };

export function setupBridge() {
  const mapStore = useMapStore();

  (window as any).__BEVY_BRIDGE_INBOUND__ = (event: InboundEvent) => {
    switch (event.type) {
      case 'PROVINCE_SELECTED':
        mapStore.selectedProvince = event.payload.province;
        mapStore.selectedState = event.payload.state;
        console.log(event.payload)
        break;
      case 'SCAN_COMPLETED':
        mapStore.provinces = event.payload.provinces;
        console.log('Scan completed:', event.payload.provinces);
        break;
    }
  };
}

export { update_brush_settings };

// En el archivo donde manejás los mensajes de Bevy a JS:
window.addEventListener('bevy_message', (event: any) => {
  const { action, payload } = (event as CustomEvent).detail;

  if (action === "STATE_ASSIGNED_TO_REGION") {
    const regionStore = useRegionStore();
    regionStore.addStateToRegion(payload.state_id, payload.region_id);
  }
});