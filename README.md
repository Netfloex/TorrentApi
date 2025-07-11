# Torrent Api

Torrent search api wrapper and parser for popular torrent sites.

### Packages

- `api-server`: The http api for querying the search package
- `torrent-search-client`: A rust crate for searching on popular torrent sites

### Usage

#### Docker Compose

```yaml
version: "3"
services:
    torrentapi:
        image: netfloex/torrent-api
        container_name: torrent-api
        restart: unless-stopped
        ports:
            - 8000:8000
        volumes:
            - ./cache:/http-cacache
```

#### Docker Run

```sh
docker run -it -p 8000:8000 -v $PWD/cache:/http-cacache netfloex/torrent-api
```

### Usage

#### Search for torrents

```sh
curl localhost:8000/search?q=<query>
```

#### Search for movie torrents

```sh
curl localhost:8000/search?imdb=<imdb_id>
```

### Parameters

#### Torrent search

| param    | description                                               | Required           |
| -------- | --------------------------------------------------------- | ------------------ |
| query    | Query to search for torrents                              | :heavy_check_mark: |
| category | `All`, `Applications`, `Audio`, `Games`, `Other`, `Video` | :x:                |
| sort     | `Added`, `Size`, `Seeders`, `Leechers`                    | :x:                |
| order    | `Asc`, `Desc`                                             | :x:                |
| limit    | Integer                                                   | :x:                |

#### Movie search

| param   | description                                                               | Required           |
| ------- | ------------------------------------------------------------------------- | ------------------ |
| imdb    | IMDB id                                                                   | :heavy_check_mark: |
| title   | Title of the movie                                                        | :x:                |
| sort    | `Added`, `Size`, `Seeders`, `Leechers`                                    | :x:                |
| order   | `Asc`, `Desc`                                                             | :x:                |
| limit   | `Integer`, limit the results to ... length                                | :x:                |
| quality | `480p`,`720p`,`1080p`,`2160p`                                             | :x:                |
| codec   | `x264`, `x265`                                                            | :x:                |
| source  | `Cam`, `Telesync`, `Telecine`, `Dvd`, `Hdtv`, `Hdrip`, `WebRip`, `BluRay` | :x:                |

### Current Providers

- BitSearch (bitsearch.to)
- The Pirate Bay (apibay.org)
- Yts (yts.mx)
