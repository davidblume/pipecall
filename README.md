# PipeCall


## Troubleshooting

    ### Error: Bluetooth operation not permitted: UUID already registered
    When starting PipeCall on modern Linux desktops, you may encounter a crash indicating the HFP UUID is already registered. This happens because the system's default audio server
  (PipeWire) aggressively claims the Hands-Free Profile to support Bluetooth headsets automatically.

    To run PipeCall alongside PipeWire without disabling your desktop audio, you need to configure PipeWire's session manager (WirePlumber) to ignore the Hands-Free roles.

    **Fix for WirePlumber (0.5+):**
    1. Create a custom configuration file:
       ```bash
       mkdir -p ~/.config/wireplumber/wireplumber.conf.d/
       nano ~/.config/wireplumber/wireplumber.conf.d/51-disable-hfp.conf

  2. Add the following configuration to restrict BlueZ to high-quality audio only:
    monitor.bluez.properties = {
      "bluez5.roles" = [ "a2dp_sink", "a2dp_source" ]
    }

  3. Restart WirePlumber:
    systemctl --user restart wireplumber


  PipeCall should now be able to claim the HFP profile successfully while PipeWire handles your normal system audio.
