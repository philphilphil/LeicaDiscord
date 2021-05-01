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
        public async Task<KenRQuote[]> GetQuotesAsync()
        {
            using var db = DbFactory.CreateDbContext();
            return await Task.FromResult(db.KenRQuotes.ToArray());
        }

        public async Task<int> DeleteQuoteAsync(KenRQuote quote)
        {
            using var db = DbFactory.CreateDbContext();
            db.Remove(quote);
            return await db.SaveChangesAsync();
        }

        public async Task<int> UpdateQuoteAsync(KenRQuote quote)
        {
            using var db = DbFactory.CreateDbContext();
            db.Update(quote);
            return await db.SaveChangesAsync();
        }

        public async Task<int> AddQuoteAsync(KenRQuote quote)
        {
            using var db = DbFactory.CreateDbContext();
            db.Add(quote);
            return await db.SaveChangesAsync();
        }
    }
}
