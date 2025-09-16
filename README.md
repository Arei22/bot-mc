
<!--suppress HtmlDeprecatedAttribute -->
<div align="center">
    <h1>
      Bot-mc
    </h1>
    <div>
        <a href="https://www.rust-lang.org/">
            <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
        </a>
    </div>
    <h3>
        <strong>A discord bot to create and start Minecraft servers</strong>
    </h3>
</div>

## Informations
This is a discord bot with which you can create, start, stop and delete servers. You can actually create a server in the version and difficulty of your choice, and you can also list all the servers created. For now there is not the possibility to use a mod, a map, or a plugin.

## Instalation
Install the bot with docker compose and the ``docker-compose.yml`` :
```yml
services:
  postgre:
    restart: always
    image: postgres:17.6-alpine3.22
    container_name: bot-mc-database
    environment:
      POSTGRES_DB: bot-mc
      POSTGRES_USER: jacques
      POSTGRES_PASSWORD: passsword
    volumes:
      - database-data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD", "pg_isready"]
      interval: 30s
      timeout: 60s
      retries: 3
      start_period: 60s

  bot-mc:
    container_name: bot-mc
    image: arei2/bot-mc
    restart: always
    environment:
      DISCORD_TOKEN: token
      DISCORD_GUILD_ID: guil_id
      DATABASE_URL: postgres://jacques:password@postgre/bot-mc
      DISCORD_APP_ID: app_i
      ADMIN_PLAYER: Arei22
      IP: ip
      MIN_PORT: 10000
      MAX_PORT: 10500
      MAX_MEMORY: 20G
      DEV_MODE: false
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    depends_on:
      postgre:
        condition: service_healthy
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 50M
        reservations:
          memory: 20M

volumes:
  database-data:
```

## Contributors
[<img width="45" src="https://avatars.githubusercontent.com/u/126862312?s=96&v=4" alt="Arei2">](https://github.com/Arei22)

## License
**[Bot-mc](https://github.com/arei22) | [GNU GENERAL PUBLIC LICENSE 3.0](https://github.com/arei22/Bot-mc/blob/main/LICENSE.txt)**
