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

TODO structure not final

5. **Run**

TODO: either as container or as binary
