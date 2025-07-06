-- wrk -t16 -c400 -d30s -s bench_transfer.lua http://localhost:8080/balance/transfer
-- autocannon -c 100 -d 30 --warmup 10 -m POST http://localhost:8080/balance/transfer

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"from_id": 1, "to_id": 2, "amount": 1}'
