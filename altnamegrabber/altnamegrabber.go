package main

import (
	"bufio"
	"crypto/tls"
	"flag"
	"fmt"
	"net"
	"net/http"
	"os"
	"sync"
	"time"
)

func getClient() *http.Client {
	tr := &http.Transport{
		MaxIdleConns:    30,
		IdleConnTimeout: time.Second,
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
		DialContext: (&net.Dialer{
			Timeout:   time.Second * 10,
			KeepAlive: time.Second,
		}).DialContext,
	}

	re := func(req *http.Request, via []*http.Request) error {
		return http.ErrUseLastResponse
	}

	return &http.Client{
		Transport:     tr,
		CheckRedirect: re,
		Timeout:       time.Second * 10,
	}
}

func sendRequest(c *http.Client, url string, certnames map[string]bool) {

	resp, err := c.Head(url)
	if err != nil {
		fmt.Println(err)
		return
	}
	certificates := resp.TLS.PeerCertificates
	if len(certificates) > 0 {
		for _, entry := range certificates[0].DNSNames {
			if certnames[entry] != true {
				certnames[entry] = true
			}
		}
	}
}

func main() {

	// command line args

	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage of %s:\nTakes a list of https urls from stdin and prints out the dns alt names form the certificate\n\n", os.Args[0])
		flag.PrintDefaults()
	}

	var threads int
	flag.IntVar(&threads, "t", 20, "set the amount of threads")
	flag.Parse()

	urls := make(chan string)
	certnames := make(map[string]bool)

	var workers sync.WaitGroup
	for i := 0; i < threads; i++ {
		workers.Add(1)
		c := getClient()
		go func() {
			defer workers.Done()
			for u := range urls {
				sendRequest(c, u, certnames)
			}
		}()
	}

	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		urls <- sc.Text()
	}
	close(urls)
	workers.Wait()

	for entry := range certnames {
		fmt.Println(entry)
	}
}
