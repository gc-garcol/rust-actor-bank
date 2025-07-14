-- wrk -t16 -c400 -d30s -s bench_transfer.lua http://localhost:8080/balance/transfer
-- autocannon -c 400 -d 60 -m POST -H "Content-Type: application/json" -b '{"from_id":1,"to_id":2,"amount":1}' --warmup [ -c 400 -d 10 ] http://localhost:8080/balance/transfer

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"from_id": 1, "to_id": 2, "amount": 1}'
