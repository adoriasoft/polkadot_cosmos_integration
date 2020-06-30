package main

import (
	"abci-grpc/src/abci"
	"flag"
	"sync"

	"github.com/golang/glog"
)

func main() {
	flag.Parse()
	defer glog.Flush()

	var wg sync.WaitGroup

	wg.Add(1)
	go func() {
		defer wg.Done()
		abci.Grpc_run()
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		abci.Grpc_http_run()
	}()

	wg.Wait()
}
