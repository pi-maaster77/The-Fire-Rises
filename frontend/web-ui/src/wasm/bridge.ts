// frontend/web-ui/src/wasm/bridge.ts
import { useMapStore } from '../stores/map'

export function setupBridge() {
  const mapStore = useMapStore();

  (window as any).__BEVY_BRIDGE_INBOUND__ = (event: any) => {
    switch (event.type) {
      case 'PROVINCE_SELECTED':
        mapStore.selectedProvince = event.payload.province;
        mapStore.selectedState = event.payload.state;
				console.log(event.payload)
        break;
      case 'PROVINCES_LIST':
        mapStore.provinces = event.payload.provinces;
        break;
    }
  };
}