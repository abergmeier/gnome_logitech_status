#ifndef __GSD_LOGITECH_MANAGER_H
#define __GSD_LOGITECH_MANAGER_H

#include <glib-object.h>

G_BEGIN_DECLS

#define GSD_TYPE_LOGITECH_MANAGER (gsd_logitech_manager_get_type ())

G_DECLARE_FINAL_TYPE (GsdLogitechManager, gsd_logitech_manager, GSD, LOGITECH_MANAGER, GObject)

GsdLogitechManager * gsd_logitech_manager_new  (void);
gboolean             gsd_logitech_manager_start(GsdLogitechManager *manager,
                                                GError         **error);
void                 gsd_logitech_manager_stop (GsdLogitechManager *manager);

G_END_DECLS

#endif /* __GSD_LOGITECH_MANAGER_H */
