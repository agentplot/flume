# A `settings` setting every table and key design 6 declares, rendered to
# nix/tests/files/every-key.toml and read whole by flume's own reader of
# the host's configuration (the host-module spec; the change
# flume-publishes-its-own-flake-and-module, design 6 and 7). It sets
# host.multiplexer and neither host.terminal nor a stream's points or
# pace, which the module does not declare, and one key no declaration
# names that flume's reader passes over. Every value is one flume reads
# without a fault: `printed` where a multiplexer or a terminal app is
# named, and a settings file, `people-settings.json`, the reader's test
# makes beside the file.
{
  host = {
    agents = 8;
    multiplexer = "printed";
    pool = "work";
    terminal_app = "printed";
    view = "terminal";
    pin = [ "streams.work.agents" ];
  };

  streams = {
    work = {
      agents = 5;
      priority = 10;
      pool = "work";
      session = "work";
      multiplexer = "printed";
      terminal_app = "printed";
      title = "Work";
      view = "app";
      labels = {
        operation = {
          reserve = 2;
          keep_free = 1;
        };
        construction.reserve = 2;
      };
      kinds.research = {
        session = "work-design";
        stages.review.session = "work-review";
      };
    };
    # A stream declared empty, written as its header.
    personal = { };
  };

  pools.work = {
    strategy = "most-left";
    threshold = 90;
    accounts = [
      {
        name = "work-1";
        program = "claude";
        env.CLAUDE_CONFIG_DIR = "$HOME/.claude-work-1";
        agents = 4;
        usage = "~/.local/state/usage/work-1.json";
      }
      {
        name = "work-2";
        program = "claude";
        env.CLAUDE_CONFIG_DIR = "~/.claude-work-2";
      }
    ];
  };

  kinds = {
    change = {
      stream = "work";
      label = "construction";
      agents = {
        model = "opus";
        effort = "high";
        mode = "edits";
        keep_open = "30m";
        instructions = "Say \"done\" when the tasks are ticked.";
        programs.claude = {
          file = "people-settings.json";
          settings = {
            cleanupPeriodDays = 30;
            includeCoAuthoredBy = false;
            sampleRate = 0.5;
            permissions.allow = [
              "Bash(git status)"
              "Read"
            ];
            hooks.Stop = [
              {
                matcher = "";
                hooks = [
                  {
                    type = "command";
                    command = "true";
                  }
                ];
              }
            ];
            mixed = [
              "a"
              { b = 1; }
              [ 2 ]
            ];
          };
        };
        stages.code = {
          model = "sonnet";
          mode.claude = "acceptEdits";
        };
      };
      # No declaration names it, and flume's reader passes over it.
      undeclared = "passes as written";
    };
    research.label = "inception";
  };

  repos = {
    "acme/acme" = {
      stream = "work";
      agents.effort = "medium";
      kinds.fix = {
        stream = "personal";
        label = "operation";
        agents = {
          effort = "low";
          stages.code.model = "sonnet";
        };
      };
    };
    "~/Code/upstream-fork".stream = "personal";
  };

  agents = {
    model = "opus";
    keep_open = "1h";
    programs.claude.settings.theme = "dark";
    streams.work.effort = "high";
  };
}
