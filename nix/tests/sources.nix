# The `sources` of the host-module spec, rendered to
# nix/tests/files/sources/<source>.toml and read whole by flume's own
# reader of the host's sources (the change
# flumes-module-writes-the-hosts-sources, design 6): `acme`, its kinds
# folder and its roles file under `~`, and `tools`, an absolute kinds
# folder and no roles.
{
  acme = {
    kinds_dir = "~/.local/share/flume/kinds/acme";
    roles = "~/.local/state/acme/flume/roles.toml";
  };
  tools.kinds_dir = "/opt/kinds/tools";
}
