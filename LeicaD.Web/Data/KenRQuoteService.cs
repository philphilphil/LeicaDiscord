using System;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Data
{
    public class KenRQuoteService
    {
        private IDbContextFactory<ApplicationDbContext> DbFactory { get; set; }
        public KenRQuoteService(IDbContextFactory<ApplicationDbContext> _dbFactory)
        {
            DbFactory = _dbFactory;
        }
        public Task<KenRQuote[]> GetQuotesAsync()
        {
            using var db = DbFactory.CreateDbContext();
            return Task.FromResult(db.KenRQuotes.ToArray());
        }
    }
}
