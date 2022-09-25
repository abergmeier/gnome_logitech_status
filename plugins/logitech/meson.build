policy = 'org.gnome.settings-daemon.plugins.logitech.policy'

policy_in = configure_file(
  input: policy + '.in.in',
  output: policy + '.in',
  configuration: plugins_conf
)

i18n.merge_file(
  input: policy_in,
  output: policy,
  po_dir: po_dir,
  install: true,
  install_dir: join_paths(gsd_datadir, 'polkit-1', 'actions')
)

sources = files(
  'main.c'
)

liblogitech = meson.get_compiler('c').find_library('liblogitech', dirs: ['/home/andreas/Projects/gnome_logitech_status/liblogitech/target/debug'])

deps = plugins_deps + [
  gtk_dep,
  libcommon_dep,
  m_dep,
  pango_dep,
  liblogitech,
]

cflags += ['-DLIBEXECDIR="@0@"'.format(gsd_libexecdir)]

executable(
  'gsd-' + plugin_name,
  sources,
  include_directories: [top_inc, data_inc],
  dependencies: deps,
  c_args: cflags,
  install: true,
  install_rpath: gsd_pkglibdir,
  install_dir: gsd_libexecdir,
)
