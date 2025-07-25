```sh
docker build -t ghcr.io/shautvast/undeepend:latest .
docker push ghcr.io/shautvast/undeepend:latest
```

```sh
#!/bin/bash
# undeepend.sh
docker run --rm -v $(pwd):/project ghcr.io/shautvast/undeepend "$@"
```