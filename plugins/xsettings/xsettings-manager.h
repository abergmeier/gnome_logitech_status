/*
 * Copyright © 2001 Red Hat, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Red Hat not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  Red Hat makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 * RED HAT DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING ALL
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL RED HAT
 * BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
 * OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN 
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * Author:  Owen Taylor, Red Hat, Inc.
 */
#ifndef XSETTINGS_MANAGER_H
#define XSETTINGS_MANAGER_H

#include <X11/Xlib.h>
#include <X11/extensions/Xfixes.h>
#include "xsettings-common.h"

typedef struct _XSettingsManager XSettingsManager;

typedef void (*XSettingsTerminateFunc)  (void *cb_data);

Bool xsettings_manager_check_running (Display *display,
				      int      screen);

XSettingsManager *xsettings_manager_new (Display                *display,
					 int                     screen,
					 XSettingsTerminateFunc  terminate,
					 void                   *cb_data);

void   xsettings_manager_destroy       (XSettingsManager *manager);

void   xsettings_manager_delete_setting (XSettingsManager *manager,
                                         const char       *name);
void   xsettings_manager_set_int        (XSettingsManager *manager,
                                         const char       *name,
                                         int               value);
void   xsettings_manager_set_string     (XSettingsManager *manager,
                                         const char       *name,
                                         const char       *value);
void   xsettings_manager_set_color      (XSettingsManager *manager,
                                         const char       *name,
                                         XSettingsColor   *value);
void   xsettings_manager_notify         (XSettingsManager *manager);
void   xsettings_manager_set_overrides  (XSettingsManager *manager,
                                         GVariant         *overrides);

#endif /* XSETTINGS_MANAGER_H */
