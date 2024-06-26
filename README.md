# domeneshop-ddns

A simple script to update a DNS record on Domeneshop.

## Usage

### Docker

```bash
docker run --name domeneshop-ddns \
  -d \
  -e DOMENESHOP_TOKEN=<your_token> \
  -e DOMENESHOP_SECRET=<your_secret> \
  -e DDNS_DOMAINS=<your_domains> \
  jnatten/domeneshop-ddns:0.1.1
```

### Docker compose

```yaml
version: '3'
services:
  domeneshop-ddns:
    container_name: domeneshop-ddns
    image: jnatten/domeneshop-ddns:0.1.1
    environment:
      - DOMENESHOP_TOKEN=<your_token>
      - DOMENESHOP_SECRET=<your_secret>
      - DDNS_DOMAINS=<your_domains>
    restart: always
```

### Manual

```bash
cargo run
```

### Configuration

The application is configured using environment variables.
These are the same whether you are running the application in a container or manually.

| Variable               | Description                                                                                                 | Required |
|------------------------|-------------------------------------------------------------------------------------------------------------|----------|
| DOMENESHOP_TOKEN       | The token for the Domeneshop API. Can be created on your [admin page.](https://domene.shop/admin?view=api)  | Yes      |
| DOMENESHOP_SECRET      | The secret for the Domeneshop API. Can be created on your [admin page.](https://domene.shop/admin?view=api) | Yes      |
| DDNS_DOMAINS           | A comma separated list of domains to update.                                                                | Yes      |
| SLEEP_INTERVAL_SECONDS | The interval in seconds to update the DNS records. Default is 60.                                           | No       |
| MYIP                   | The IP address to update the DNS records to. Default is the public IP of the host.                          | No       |

