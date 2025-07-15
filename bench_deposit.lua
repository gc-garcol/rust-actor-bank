-- wrk -t16 -c400 -d30s -s bench_deposit.lua http://localhost:8080/balance/deposit
-- autocannon -c 400 -d 60 -m POST -H "Content-Type: application/json" -b '{"id":1,"amount":1}' --warmup [ -c 400 -d 10 ] http://localhost:8080/balance/deposit

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"id":1,"amount":1}'
