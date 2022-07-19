using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Order;
using Benchmarks;
using Grpc.Core;
using Grpc.Net.Client;
using System;
using System.Runtime.InteropServices;
using System.Security;

[MemoryDiagnoser]
[Orderer(SummaryOrderPolicy.FastestToSlowest, MethodOrderPolicy.Declared)]
[SimpleJob(launchCount: 2, warmupCount: 4, targetCount: 6)]
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

    [SuppressUnmanagedCodeSecurity]
    [DllImport(FFIDJI.InterfaceStrings.LIBRARY_NAME, EntryPoint = "ConcatNoFFIDJI", CallingConvention = CallingConvention.Cdecl)]
    private extern static IntPtr ConcatNoFFIDJI_FFI(string a, string b);

    [Benchmark]
    public string ConcatNoFFIDJI()
    {
        var ptr = ConcatNoFFIDJI_FFI(_a, _b);
        return Marshal.PtrToStringAuto(ptr);
    }

    [Benchmark]
    public void NoArgs()
    {
        // No computations, no args, no returns. Just to see the cost of calling native code from csharp
        FFIDJI.InterfaceStrings.NoArgs();
    }
}