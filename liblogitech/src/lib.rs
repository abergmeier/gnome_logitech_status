use crate::protocol::LitraDevices;
use glib;
use glib_sys;
use gobject_sys;
use libc::c_char;

use glib::prelude::*;
use glib::subclass::prelude::*;

mod protocol;
/*
#define UNKNOWN_DEVICE_NOTIFICATION_TIMEOUT 15000

*/
const GSD_DBUS_NAME: &str = "org.gnome.SettingsDaemon";
const GSD_DBUS_PATH: &str = "/org/gnome/SettingsDaemon";
/*
#define GSD_DBUS_BASE_INTERFACE "org.gnome.SettingsDaemon"

#define GSD_WACOM_DBUS_PATH GSD_DBUS_PATH "/Logitech"
*/
const GSD_LOGITECH_DBUS_NAME: &str = (GSD_DBUS_NAME.to_owned() + ".Logitech").as_str();
/*
#define LEFT_HANDED_KEY		"left-handed"
*/

const GSD_LOGITECH_DBUS_PATH: &str = (GSD_DBUS_PATH.to_owned() + "/Logitech").as_str();

const INTROSPECTION_XML: &str = r#"
<node name='/org/gnome/SettingsDaemon/Logitech'>
    <interface name='org.gnome.SettingsDaemon.Logitech'>
        <method name='SetBrightness'>
            <arg name='level' direction='in' type='i'/>
        </method>
    </interface>
</node>"#;

glib::wrapper! {
    pub struct GsdLogitechManager(ObjectSubclass<imp::Manager>);
}

mod imp {

    use gio::BusNameOwnerFlags;

    use super::*;
    #[derive(Default)]
    pub struct Manager {
        devices: Option<LitraDevices>,
        // DBus
        introspection_data: Option<gio::DBusNodeInfo>,
        dbus_cancellable: Option<gio::Cancellable>,
        dbus_connection: Option<gio::DBusConnection>,
        dbus_register_object_id: Option<gio::RegistrationId>,
        name_id: Option<gio::OwnerId>,
        /*
        guint            name_id;
        */
    }

    impl Manager {
        fn init(self) {
            self.devices = Some(LitraDevices::new());
        }
        fn register(&self) {
            self.introspection_data = Some(gio::DBusNodeInfo::for_xml(INTROSPECTION_XML).unwrap());
            self.dbus_cancellable = Some(gio::Cancellable::new());

            gio::bus_get(
                gio::BusType::Session,
                self.dbus_cancellable.as_ref(),
                |res| self.on_bus_gotten(res),
            );
            /*

                    g_bus_get (G_BUS_TYPE_SESSION,
                        self->dbus_cancellable,
                        (GAsyncReadyCallback) on_bus_gotten,
                        self);
            */
        }

        fn on_bus_gotten(&self, res: Result<gio::DBusConnection, glib::Error>) {
            /*
            GDBusConnection	       *connection;
            GError		       *error = NULL;

            connection = g_bus_get_finish (res, &error);
            */
            if res.is_err() {
                let error = res.unwrap_err();
                if !error.matches(gio::IOErrorEnum::Cancelled) {
                    glib::g_warning!("logitech", "Couldn't get session bus: {}", error.message());
                }
                return;
            }

            let connection = res.unwrap();

            self.dbus_connection = Some(connection);
            let handle_method_call = |conn: gio::DBusConnection,
                                      sender: &str,
                                      object_path: &str,
                                      interface_name: &str,
                                      method_name: &str,
                                      parameters: glib::Variant,
                                      invocation| {
                (&self).handle_method_call(
                    conn,
                    sender,
                    object_path,
                    interface_name,
                    method_name,
                    parameters,
                    invocation,
                )
            };
            let get_property = |conn,
                                sender: &str,
                                object_path: &str,
                                interface_name: &str,
                                property_name: &str| {
                glib::Variant::from_none(&glib::VariantDict::static_variant_type())
            };
            let set_property = |conn,
                                sender: &str,
                                object_path: &str,
                                interface_name: &str,
                                property_name: &str,
                                value: glib::Variant| { false };
            let introspect_data = self.introspection_data.unwrap();
            let ifaces: &[gio::DBusInterfaceInfo] = introspect_data.interfaces();
            if ifaces.is_empty() {
                // TODO: Log that no interfaces where to be head
                return;
            } else {
                self.dbus_register_object_id = Some(
                    self.dbus_connection
                        .unwrap()
                        .register_object(
                            GSD_LOGITECH_DBUS_PATH,
                            &ifaces[0],
                            handle_method_call,
                            get_property,
                            set_property,
                        )
                        .unwrap(),
                );
            }

            let name_aquired = |conn: gio::DBusConnection, name: &str| {};
            let name_lost = |conn: gio::DBusConnection, name: &str| {};
            self.name_id = Some(gio::bus_own_name_on_connection(
                &connection,
                GSD_LOGITECH_DBUS_NAME,
                BusNameOwnerFlags::empty(),
                name_aquired,
                name_lost,
            ))
        }

        fn handle_method_call(
            &self,
            connection: gio::DBusConnection,
            sender: &str,
            object_path: &str,
            interface_name: &str,
            method_name: &str,
            parameters: glib::Variant,
            invocation: gio::DBusMethodInvocation,
        ) {
            /*
            GsdWacomManager *self = GSD_WACOM_MANAGER (data);
                GError *error = NULL;
                GdkDevice *device;

                if (g_strcmp0 (method_name, "SetOLEDLabels") == 0) {
                        gchar *device_path, *label;
                        gboolean left_handed;
                        GSettings *settings;
                        GVariantIter *iter;
                        gint i = 0;

                g_variant_get (parameters, "(sas)", &device_path, &iter);
                        device = lookup_device_by_path (self, device_path);
                        if (!device) {
                                g_dbus_method_invocation_return_value (invocation, NULL);
                                return;
                        }

                        settings = device_get_settings (device);
                        left_handed = g_settings_get_boolean (settings, LEFT_HANDED_KEY);
                        g_object_unref (settings);

                        while (g_variant_iter_loop (iter, "s", &label)) {
                                if (!set_oled (device_path, left_handed, i, label, &error)) {
                                        g_free (label);
                                        break;
                                }
                                i++;
                        }

                        g_variant_iter_free (iter);

                        if (error)
                                g_dbus_method_invocation_return_gerror (invocation, error);
                        else
                                g_dbus_method_invocation_return_value (invocation, NULL);
                }
                */
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Manager {
        // This type name must be unique per process.
        const NAME: &'static str = "GsdLogitechManager";

        type Type = super::GsdLogitechManager;

        // The parent type this one is inheriting from.
        // Optional, if not specified it defaults to `glib::Object`
        type ParentType = glib::Object;

        // Interfaces this type implements.
        // Optional, if not specified it defaults to `()`
        type Interfaces = ();
    }

    impl ObjectImpl for Manager {}
}

/*
struct _GsdWacomManager
{
        GObject parent;

        guint start_idle_id;
        GdkSeat *seat;
        guint device_added_id;

        GsdShell *shell_proxy;

        gchar *machine_id;

#if HAVE_WACOM
        WacomDeviceDatabase *wacom_db;
#endif

        /* DBus */
        GDBusNodeInfo   *introspection_data;
        GDBusConnection *dbus_connection;
        GCancellable    *dbus_cancellable;
        guint            dbus_register_object_id;
        guint            name_id;
};

static void     gsd_wacom_manager_class_init  (GsdWacomManagerClass *klass);
static void     gsd_wacom_manager_init        (GsdWacomManager      *wacom_manager);
static void     gsd_wacom_manager_finalize    (GObject              *object);

static gboolean is_opaque_tablet (GsdWacomManager *manager,
                                  GdkDevice       *device);

G_DEFINE_TYPE (GsdWacomManager, gsd_wacom_manager, G_TYPE_OBJECT)

static GVariant *
map_tablet_mapping (GVariant *value, GVariant *old_default, GVariant *new_default)
{
        const gchar *mapping;

        mapping = g_variant_get_boolean (value) ? "absolute" : "relative";
        return g_variant_new_string (mapping);
}

static GVariant *
map_tablet_left_handed (GVariant *value, GVariant *old_default, GVariant *new_default)
{
        const gchar *rotation = g_variant_get_string (value, NULL);
        return g_variant_new_boolean (g_strcmp0 (rotation, "half") == 0 ||
                                      g_strcmp0 (rotation, "ccw") == 0);
}

static void
migrate_tablet_settings (GsdWacomManager *manager,
                         GdkDevice       *device)
{
        GsdSettingsMigrateEntry tablet_settings[] = {
                { "is-absolute", "mapping", map_tablet_mapping },
                { "keep-aspect", "keep-aspect", NULL },
                { "rotation", "left-handed", map_tablet_left_handed },
        };
        gchar *old_path, *new_path;
        const gchar *vendor, *product;

        vendor = gdk_device_get_vendor_id (device);
        product = gdk_device_get_product_id (device);

        old_path = g_strdup_printf ("/org/gnome/settings-daemon/peripherals/wacom/%s-usb:%s:%s/",
                                    manager->machine_id, vendor, product);
        new_path = g_strdup_printf ("/org/gnome/desktop/peripherals/tablets/%s:%s/",
                                    vendor, product);

        gsd_settings_migrate_check ("org.gnome.settings-daemon.peripherals.wacom.deprecated",
                                    old_path,
                                    "org.gnome.desktop.peripherals.tablet",
                                    new_path,
                                    tablet_settings, G_N_ELEMENTS (tablet_settings));

        /* Opaque tablets' mapping may be modified by users, so only these
         * need migration of settings.
         */
        if (is_opaque_tablet (manager, device)) {
                GsdSettingsMigrateEntry display_setting[] = {
                        { "display", "output", NULL },
                };

                gsd_settings_migrate_check ("org.gnome.desktop.peripherals.tablet.deprecated",
                                            new_path,
                                            "org.gnome.desktop.peripherals.tablet",
                                            new_path,
                                            display_setting, G_N_ELEMENTS (display_setting));
        }

        g_free (old_path);
        g_free (new_path);
}

static void
gsd_wacom_manager_class_init (GsdWacomManagerClass *klass)
{
        GObjectClass   *object_class = G_OBJECT_CLASS (klass);

        object_class->finalize = gsd_wacom_manager_finalize;
}

static gchar *
get_device_path (GdkDevice *device)
{
#if HAVE_WAYLAND
        if (gnome_settings_is_wayland ())
                return g_strdup (gdk_wayland_device_get_node_path (device));
        else
#endif
                return xdevice_get_device_node (gdk_x11_device_get_id (device));
}

static gboolean
is_opaque_tablet (GsdWacomManager *manager,
                  GdkDevice       *device)
{
        gboolean is_opaque = FALSE;
#if HAVE_WACOM
        WacomDevice *wacom_device;
        gchar *devpath;

        devpath = get_device_path (device);
        wacom_device = libwacom_new_from_path (manager->wacom_db, devpath,
                                               WFALLBACK_GENERIC, NULL);
        if (wacom_device) {
                WacomIntegrationFlags integration_flags;

                integration_flags = libwacom_get_integration_flags (wacom_device);
                is_opaque = (integration_flags &
                             (WACOM_DEVICE_INTEGRATED_DISPLAY | WACOM_DEVICE_INTEGRATED_SYSTEM)) == 0;
                libwacom_destroy (wacom_device);
        }

#endif
        return is_opaque;
}

static GdkDevice *
lookup_device_by_path (GsdWacomManager *manager,
                       const gchar     *path)
{
        GList *devices, *l;

        devices = gdk_seat_get_slaves (manager->seat,
                                       GDK_SEAT_CAPABILITY_ALL);

        for (l = devices; l; l = l->next) {
                GdkDevice *device = l->data;
                gchar *dev_path = get_device_path (device);

                if (g_strcmp0 (dev_path, path) == 0) {
                        g_free (dev_path);
                        return device;
                }

                g_free (dev_path);
        }

        g_list_free (devices);

        return NULL;
}

static GSettings *
device_get_settings (GdkDevice *device)
{
        GSettings *settings;
        gchar *path;

        path = g_strdup_printf ("/org/gnome/desktop/peripherals/tablets/%s:%s/",
                                gdk_device_get_vendor_id (device),
                                gdk_device_get_product_id (device));
        settings = g_settings_new_with_path ("org.gnome.desktop.peripherals.tablet",
                                             path);
        g_free (path);

        return settings;
}
*/

/*
static const GDBusInterfaceVTable interface_vtable =
{
    handle_method_call,
    NULL, /* Get Property */
    NULL, /* Set Property */
};

static void
device_added_cb (GdkSeat         *seat,
                 GdkDevice       *device,
                 GsdWacomManager *manager)
{
        if (gdk_device_get_source (device) == GDK_SOURCE_PEN &&
            gdk_device_get_device_type (device) == GDK_DEVICE_TYPE_SLAVE) {
                migrate_tablet_settings (manager, device);
        }
}

static void
add_devices (GsdWacomManager     *manager,
             GdkSeatCapabilities  capabilities)
{
        GList *devices, *l;

        devices = gdk_seat_get_slaves (manager->seat, capabilities);
        for (l = devices; l ; l = l->next)
        device_added_cb (manager->seat, l->data, manager);
        g_list_free (devices);
}

static void
set_devicepresence_handler (GsdWacomManager *manager)
{
        GdkSeat *seat;

        seat = gdk_display_get_default_seat (gdk_display_get_default ());
        manager->device_added_id = g_signal_connect (seat, "device-added",
                                                           G_CALLBACK (device_added_cb), manager);
        manager->seat = seat;
}

*/

impl GsdLogitechManager {
    pub fn new() -> Self {
        return glib::Object::new::<GsdLogitechManager>(&[]);
    }
    pub fn init(&self) {}
}

/*

 static gboolean
 gsd_wacom_manager_idle_cb (GsdWacomManager *manager)
 {
         gnome_settings_profile_start (NULL);

         set_devicepresence_handler (manager);

         add_devices (manager, GDK_SEAT_CAPABILITY_TABLET_STYLUS);

         gnome_settings_profile_end (NULL);

         manager->start_idle_id = 0;

         return FALSE;
 }



 static gchar *
 get_machine_id (void)
 {
         gchar *no_per_machine_file, *machine_id = NULL;
         gboolean per_machine;
         gsize len;

         no_per_machine_file = g_build_filename (g_get_user_config_dir (), "gnome-settings-daemon", "no-per-machine-config", NULL);
         per_machine = !g_file_test (no_per_machine_file, G_FILE_TEST_EXISTS);
         g_free (no_per_machine_file);

         if (!per_machine ||
             (!g_file_get_contents ("/etc/machine-id", &machine_id, &len, NULL) &&
              !g_file_get_contents ("/var/lib/dbus/machine-id", &machine_id, &len, NULL))) {
                 return g_strdup ("00000000000000000000000000000000");
         }

         machine_id[len - 1] = '\0';
         return machine_id;
 }

*/

#[no_mangle]
pub extern "C" fn gsd_logitech_manager_start(
    manager: *mut gobject_sys::GObject,
    error: *mut *mut glib_sys::GError,
) -> bool {
    gnome_settings_profile_start("gsd_logitech_manager_start");

    /*
    manager.register();

    manager->machine_id = get_machine_id ();

    manager->start_idle_id = g_idle_add ((GSourceFunc) gsd_wacom_manager_idle_cb, manager);
    g_source_set_name_by_id (manager->start_idle_id, "[gnome-settings-daemon] gsd_wacom_manager_idle_cb");

    */
    gnome_settings_profile_end("gsd_logitech_manager_start");
    return true;
}

#[no_mangle]
pub extern "C" fn gsd_logitech_manager_stop(manager: *mut GsdLogitechManager) {
    /*
    g_debug ("Stopping wacom manager");

    g_clear_pointer (&manager->machine_id, g_free);

         if (manager->name_id != 0) {
                 g_bus_unown_name (manager->name_id);
                 manager->name_id = 0;
         }

         if (manager->dbus_register_object_id) {
                 g_dbus_connection_unregister_object (manager->dbus_connection,
                    manager->dbus_register_object_id);
                    manager->dbus_register_object_id = 0;
         }

         if (manager->seat != NULL) {
                 g_signal_handler_disconnect (manager->seat, manager->device_added_id);
                 manager->seat = NULL;
         }
         */
}

/*
fn gsd_logitech_manager_finalize (GObject *object) {
         GsdWacomManager *wacom_manager;

         g_return_if_fail (object != NULL);
         g_return_if_fail (GSD_IS_WACOM_MANAGER (object));

         wacom_manager = GSD_WACOM_MANAGER (object);

         g_return_if_fail (wacom_manager != NULL);

         gsd_wacom_manager_stop (wacom_manager);

         if (wacom_manager->start_idle_id != 0)
                 g_source_remove (wacom_manager->start_idle_id);

         g_clear_object (&wacom_manager->shell_proxy);

 #if HAVE_WACOM
         libwacom_database_destroy (wacom_manager->wacom_db);
 #endif

         G_OBJECT_CLASS (gsd_wacom_manager_parent_class)->finalize (object);
 }
*/

#[no_mangle]
pub extern "C" fn gsd_logitech_manager_new() -> *mut gobject_sys::GObject {
    let m = GsdLogitechManager::new();
    return m.as_ptr().cast();
}

extern "C" {
    pub fn _gnome_settings_profile_log(
        func: *const c_char,
        note: *const c_char,
        format: *const c_char,
    );
}

fn gnome_settings_profile_start(function_name: &str) {
    unsafe {
        let c_str = std::ffi::CString::new(function_name).unwrap();
        _gnome_settings_profile_log(
            c_str.as_ptr().cast(),
            "start".as_ptr().cast(),
            std::ptr::null(),
        );
    }
}

fn gnome_settings_profile_end(function_name: &str) {
    unsafe {
        let c_str = std::ffi::CString::new(function_name).unwrap();
        _gnome_settings_profile_log(
            c_str.as_ptr().cast(),
            "end".as_ptr().cast(),
            std::ptr::null(),
        );
    }
}
