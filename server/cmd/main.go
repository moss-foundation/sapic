package main

import (
	"log/slog"
	"net/http"
	"os"
	"sapic-server/internal/domain/bootstrap"
	"sapic-server/internal/domain/healthcheck"

	"github.com/valyala/fasthttp"
)

const (
	DEFAULT_PORT = ":8080"
)

const (
	API_V1_HEALTHCHECK = "/api/v1/healthcheck"
	API_V1_BOOTSTRAP   = "/api/v1/bootstrap"
)

func main() {
	var (
		port = DEFAULT_PORT
	)

	if p := os.Getenv("SERVER_PORT"); p != "" {
		port = ":" + p
	}

	slog.Info("Server is starting", slog.String("port", port))

	server := fasthttp.Server{
		Handler: router,
	}

	if err := server.ListenAndServe(port); err != nil {
		slog.Error("Failed to start server", slog.String("error", err.Error()))
		os.Exit(1)
	}
}

func router(ctx *fasthttp.RequestCtx) {
	var (
		path   = string(ctx.Path())
		method = string(ctx.Method())
	)

	switch {
	case path == API_V1_HEALTHCHECK && method == http.MethodGet:
		healthcheck.HealthCheckHandler(ctx)
	case path == API_V1_BOOTSTRAP && method == http.MethodPost:
		bootstrap.BootstrapHandler(ctx)
	default:
		ctx.SetStatusCode(http.StatusMethodNotAllowed)
	}
}
