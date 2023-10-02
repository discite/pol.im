package main

import "github.com/savsgio/atreugo/v11"

func main() {
	config := atreugo.Config{
		Addr: "0.0.0.0:8000",
	}
	server := atreugo.New(config)

	server.GET("/", func(rc *atreugo.RequestCtx) error {
		return rc.TextResponse("Hello World")
	})

	server.GET("/echo/{path:*}", func(rc *atreugo.RequestCtx) error {
		return rc.TextResponse("Echo message: " + rc.UserValue("path").(string))
	})

	v1 := server.NewGroupPath("/v1")
	v1.GET("/", func(rc *atreugo.RequestCtx) error {
		return rc.TextResponse("Hello v1 Group")
	})

	err := server.ListenAndServe()
	if err != nil {
		panic(err)
	}
}
