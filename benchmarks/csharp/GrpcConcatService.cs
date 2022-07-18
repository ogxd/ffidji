using Benchmarks;
using Grpc.Core;
using System.Threading.Tasks;

public class GrpcConcatService : ConcatService.ConcatServiceBase
{
    public override Task<ConcatResult> Submit(ConcatQuery query, ServerCallContext context)
    {
        return Task.FromResult(new ConcatResult { Ab = query.B + query.B });
    }
}