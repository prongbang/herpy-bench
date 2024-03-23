load_test_direct:
	rewrk -h http://127.0.0.1:8000/v1/hello -t 12 -c 100 -d 60s