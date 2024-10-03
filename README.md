# discord-role-icon-bot
A specialized bot that dynamically appends icons to user server-nicknames based on assigned role. Enhances server visual hierarchy in voice channels

### Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/wolfedme/discord-role-icon-bot
    cd discord-role-icon-bot
    ```

2. **Build**

    ```bash
    cargo build
    ```

    TODO add makefile

3. **Create a `.env` file:**

    In the root directory of the project, create a file named `.env` and add your [Discord API](https://discord.com/developers/applications) token:

    ```.env
    DISCORD_API_TOKEN=your-discord-api-token
    ```

    Replace `your-discord-api-token` with your actual Discord API token. See `.env.example` for reference

4. **Create a `configuration.json` file:**

Either `cargo run` once to generate a `configuration.json` file and have the path output in the logs or create one manually in `<system_config_dir>/discord-role-icon-bot/configuration.json`

See `configuration.json.example` for reference

  ```json
  {
      "roles": [
          {
              "role_id": "role_id",
              "name": "role_name",
              "symbol": "symbol"
          }
      ]
  }
  ```
    Replace `role_id`, `role_name`, and `symbol` with the actual values. The `role_id` can be found by right-clicking on the role in the Discord client and selecting "Copy ID".

5. **Run**

TODO: either as container or as binary
