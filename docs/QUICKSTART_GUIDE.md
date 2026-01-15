# HyperSpot Server - Quickstart Guide

Copy-paste commands to explore HyperSpot's API. For project overview, see [README.md](../README.md).

---

## Start the Server

```bash
cargo run --bin hyperspot-server --features users-info-example,tenant-resolver-example -- --config config/quickstart.yaml run
```

Server runs on `http://127.0.0.1:8087`. Open a **new terminal** to test.

---

## Health & OpenAPI

```bash
# Health check (JSON)
curl -s http://127.0.0.1:8087/health | python3 -m json.tool
# {"status": "healthy", "timestamp": "2026-01-15T15:01:02.000Z"}

# Kubernetes liveness probe
curl -s http://127.0.0.1:8087/healthz
# ok

# OpenAPI 3.1 spec
curl -s http://127.0.0.1:8087/openapi.json | python3 -m json.tool | head -50
```

**Interactive docs:** http://127.0.0.1:8087/docs

---

## File Parser

Parses PDF, DOCX, HTML, Markdown, images, and more into structured document blocks.

```bash
# List supported file types
curl -s http://127.0.0.1:8087/file-parser/v1/info | python3 -m json.tool
```

**Output:**
```json
{
    "supported_extensions": {
        "plain_text": ["txt", "log", "md"],
        "html": ["html", "htm"],
        "pdf": ["pdf"],
        "docx": ["docx"],
        "image": ["png", "jpg", "jpeg", "webp", "gif"],
        "generic_stub": ["doc", "rtf", "odt", "xls", "xlsx", "ppt", "pptx"]
    }
}
```

```bash
# Parse a local file
curl -s -X POST http://127.0.0.1:8087/file-parser/v1/parse-local \
  -H "Content-Type: application/json" \
  -d '{"file_path": "'$PWD'/README.md"}' | python3 -m json.tool | head -40

# Parse a file from URL
curl -s -X POST http://127.0.0.1:8087/file-parser/v1/parse-url \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/document.pdf"}' | python3 -m json.tool | head -40

# Upload and parse a file
echo "Hello, HyperSpot!" > /tmp/test.txt
curl -s -X POST "http://127.0.0.1:8087/file-parser/v1/upload?filename=test.txt" \
  -H "Content-Type: application/octet-stream" \
  --data-binary @/tmp/test.txt | python3 -m json.tool
```

**Output:**
```json
{
    "document": {
        "id": "019bc231-fcfd-7df3-a49c-82174973ec44",
        "title": "test.txt",
        "meta": {
            "source": {"type": "uploaded", "original_name": "test.txt"},
            "content_type": "text/plain"
        },
        "blocks": [
            {
                "type": "paragraph",
                "inlines": [{"type": "text", "text": "Hello, HyperSpot!", "style": {}}]
            }
        ]
    }
}
```

```bash
# Stream parsed content as Markdown
curl -s -X POST http://127.0.0.1:8087/file-parser/v1/parse-local/markdown \
  -H "Content-Type: application/json" \
  -d '{"file_path": "'$PWD'/README.md"}'
```

---

## Nodes Registry

Provides hardware and system information for all running HyperSpot nodes.

```bash
# List all nodes
curl -s http://127.0.0.1:8087/nodes-registry/v1/nodes | python3 -m json.tool
```

**Output:**
```json
[
    {
        "id": "35b975fc-3c13-c04e-d62a-43c7623895e5",
        "hostname": "your-hostname",
        "ip_address": "192.168.1.100",
        "created_at": "2026-01-15T15:01:02.000Z",
        "updated_at": "2026-01-15T15:01:02.000Z"
    }
]
```

```bash
# Get node by ID
NODE_ID=$(curl -s http://127.0.0.1:8087/nodes-registry/v1/nodes | python3 -c "import sys,json; print(json.load(sys.stdin)[0]['id'])")
curl -s "http://127.0.0.1:8087/nodes-registry/v1/nodes/$NODE_ID" | python3 -m json.tool

# Get system info (OS, CPU, memory, GPUs)
curl -s "http://127.0.0.1:8087/nodes-registry/v1/nodes/$NODE_ID/sysinfo" | python3 -m json.tool
```

**Output:**
```json
{
    "node_id": "35b975fc-3c13-c04e-d62a-43c7623895e5",
    "os": {"name": "Ubuntu", "version": "24.04", "arch": "x86_64"},
    "cpu": {"model": "Intel Core i7-1165G7", "num_cpus": 8, "cores": 4, "frequency_mhz": 2803.0},
    "memory": {"total_bytes": 16624349184, "used_bytes": 9171423232, "used_percent": 55},
    "host": {"hostname": "your-hostname", "uptime_seconds": 26268},
    "gpus": [],
    "collected_at": "2026-01-15T15:05:11.234Z"
}
```

```bash
# Get system capabilities (structured hardware/software capabilities)
curl -s "http://127.0.0.1:8087/nodes-registry/v1/nodes/$NODE_ID/syscap" | python3 -m json.tool
```

**Output:**
```json
{
    "node_id": "35b975fc-3c13-c04e-d62a-43c7623895e5",
    "capabilities": [
        {"key": "hardware:ram", "category": "hardware", "name": "ram", "present": true, "amount": 15.48, "amount_dimension": "GB"},
        {"key": "hardware:cpu", "category": "hardware", "name": "cpu", "present": true, "amount": 4.0, "amount_dimension": "cores"},
        {"key": "os:linux", "category": "os", "name": "linux", "present": true, "version": "24.04"}
    ]
}
```

---

## Tenant Resolver

Multi-tenant hierarchy management. Tenants form a tree structure with parent/child relationships.

> Requires `--features tenant-resolver-example`

```bash
# Get root tenant
curl -s http://127.0.0.1:8087/tenant-resolver/v1/root | python3 -m json.tool
```

**Output:**
```json
{
    "id": "00000000000000000000000000000001",
    "parentId": "",
    "status": "ACTIVE",
    "isAccessibleByParent": true
}
```

```bash
# List all tenants
curl -s http://127.0.0.1:8087/tenant-resolver/v1/tenants | python3 -m json.tool
```

**Output:**
```json
{
    "items": [
        {"id": "00000000000000000000000000000001", "parentId": "", "status": "ACTIVE"},
        {"id": "00000000000000000000000000000010", "parentId": "00000000000000000000000000000001", "status": "ACTIVE"}
    ],
    "page_info": {"next_cursor": null, "prev_cursor": null, "limit": 100}
}
```

```bash
# Get children of a tenant (recursive)
TENANT_ID="00000000000000000000000000000001"
curl -s "http://127.0.0.1:8087/tenant-resolver/v1/tenants/$TENANT_ID/children" | python3 -m json.tool

# Get parent chain of a tenant
curl -s "http://127.0.0.1:8087/tenant-resolver/v1/tenants/$TENANT_ID/parents" | python3 -m json.tool
```

---

## Types Registry

GTS (Global Type System) schema registry. Stores JSON schemas with hierarchical IDs.

```bash
# List all GTS entities (schemas + instances)
curl -s http://127.0.0.1:8087/types-registry/v1/entities | python3 -m json.tool | head -50

# Get a specific GTS entity by ID
curl -s "http://127.0.0.1:8087/types-registry/v1/entities/gts.x.core.modkit.plugin.v1~" | python3 -m json.tool
```

---

## Stop the Server

```bash
pkill -f hyperspot-server
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Port 8087 in use | `pkill -f hyperspot-server` |
| Empty tenant-resolver | Start with `--features tenant-resolver-example` |
| Connection refused | Server not running |

---

## Further Reading

- [ARCHITECTURE_MANIFEST.md](ARCHITECTURE_MANIFEST.md) - Architecture principles
- [MODKIT_UNIFIED_SYSTEM.md](MODKIT_UNIFIED_SYSTEM.md) - Module system
- [../guidelines/NEW_MODULE.md](../guidelines/NEW_MODULE.md) - Create modules
