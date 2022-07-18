using BenchmarkDotNet.Attributes;
using Benchmarks;
using Grpc.Core;
using Grpc.Net.Client;
using System;

[MemoryDiagnoser]
public class Benchmark
{
    private string _a;
    private string _b;
    private GrpcChannel _channel;
    private ConcatService.ConcatServiceClient _client;

    [GlobalSetup]
    public void GlobalSetup()
    {
        _a = "Hello";
        _b = "World";

        int port = Random.Shared.Next(10042, 50000);

        var serverStub = new GrpcConcatService();

        var server = new Grpc.Core.Server
        {
            Services = { ConcatService.BindService(serverStub) },
            Ports = { new ServerPort("localhost", port, ServerCredentials.Insecure) }
        };
        server.Start();

        _channel = GrpcChannel.ForAddress($"http://localhost:{port}");
        _client = new ConcatService.ConcatServiceClient(_channel);
    }

    [GlobalCleanup]
    public void GlobalCleanup()
    {
        _channel.Dispose();
    }

    [Benchmark(Baseline = true)]
    public string ConcatManaged()
    {
        return _a + _b;
    }

    [Benchmark]
    public string ConcatProtobufGrpc()
    {
        var query = new ConcatQuery();
        query.A = _a;
        query.B = _b;
        return _client.Submit(query).Ab;
    }

    [Benchmark]
    public string ConcatOptimized()
    {
        return FFIDJI.InterfaceStrings.ConcatOptimized(_a, _b);
    }
}