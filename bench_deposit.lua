-- wrk -t16 -c400 -d30s -s bench_deposit.lua http://localhost:8080/balance/deposit
-- autocannon -c 100 -d 30 --warmup 10 -m POST http://localhost:8080/balance/deposit

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"id":1,"amount":1}'
