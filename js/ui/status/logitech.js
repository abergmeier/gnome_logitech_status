// -*- mode: js; js-indent-level: 4; indent-tabs-mode: nil -*-
/* exported Indicator */

const { Gio, GObject, St } = imports.gi;

const PanelMenu = imports.ui.panelMenu;
const PopupMenu = imports.ui.popupMenu;
const Slider = imports.ui.slider;

const { loadInterfaceXML } = imports.misc.fileUtils;

const BUS_NAME = 'org.gnome.SettingsDaemon.Logitech';
const OBJECT_PATH = '/org/gnome/SettingsDaemon/Logitech';

const GlowInterface = loadInterfaceXML('org.gnome.SettingsDaemon.Logitech.Litra.Glow');
const GlowProxy = Gio.DBusProxy.makeProxyWrapper(GlowInterface);

const MaxTemperatureDelta = 6500-2700

var Indicator = GObject.registerClass(
class Indicator extends PanelMenu.SystemIndicator {
    _init() {
        super._init();
        this._proxy = new GlowProxy(Gio.DBus.session, BUS_NAME, OBJECT_PATH,
                                          (proxy, error) => {
                                              if (error) {
                                                  log(error.message);
                                                  return;
                                              }

                                              this._proxy.connect('g-properties-changed', this._sync.bind(this));
                                              this._sync();
                                          });

        this._item = new PopupMenu.PopupBaseMenuItem({ activate: false });
        this.menu.addMenuItem(this._item);

        this._brightnessSlider = new Slider.Slider(0);
        this._brightnessSliderChangedId = this._brightnessSlider.connect('notify::value',
        this._brightnessSliderChanged.bind(this));
        this._brightnessSlider.accessible_name = _("Brightness");

        this._temperatureSlider = new Slider.Slider(0);
        this._temperatureSliderChangedId = this._temperatureSlider.connect('notify::value',
        this._temperatureSliderChanged.bind(this));
        this._temperatureSlider.accessible_name = _("Temperature");

        const brightnessIcon = new St.Icon({
            icon_name: 'display-brightness-symbolic',
            style_class: 'popup-menu-icon',
        });
        this._item.add(brightnessIcon);
        this._item.add_child(this._brightnessSlider);
        this._item.connect('button-press-event', (actor, event) => {
            return this._brightnessSlider.startDragging(event);
        });
        this._item.connect('key-press-event', (actor, event) => {
            return this._brightnessSlider.emit('key-press-event', event);
        });
        this._item.connect('scroll-event', (actor, event) => {
            return this._brightnessSlider.emit('scroll-event', event);
        });

        const temperatureIcon = new St.Icon({
            icon_name: 'display-temperature-symbolic',
            style_class: 'popup-menu-icon',
        });
        this._item.add(temperatureIcon);
        this._item.add_child(this._temperatureSlider);
        this._item.connect('button-press-event', (actor, event) => {
            return this._temperatureSlider.startDragging(event);
        });
        this._item.connect('key-press-event', (actor, event) => {
            return this._temperatureSlider.emit('key-press-event', event);
        });
        this._item.connect('scroll-event', (actor, event) => {
            return this._temperatureSlider.emit('scroll-event', event);
        });
    }

    _brightnessSliderChanged() {
        let percent = this._brightnessSlider.value * 100;
        this._proxy.Brightness = percent;
    }

    _temperatureSliderChanged() {
        this._proxy.Temperature = 2700 * this._temperatureSlider.value
    }

    _changeBrightnessSlider(value) {
        this._brightnessSlider.block_signal_handler(this._brightnessSliderChangedId);
        this._brightnessSlider.value = value;
        this._brightnessSlider.unblock_signal_handler(this._brightnessSliderChangedId);
    }

    _sync() {
        let visible = this._proxy.Brightness >= 0;
        this._item.visible = visible;
        if (visible)
            this._changeBrightnessSlider(this._proxy.Brightness / 100.0);
    }
});
