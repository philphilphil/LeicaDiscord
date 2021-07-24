// using System;
// using System.IO;
// using System.Linq;
// using System.Threading.Tasks;
// using Microsoft.AspNetCore.Authorization;
// using Microsoft.EntityFrameworkCore;

// namespace LeicaD.Web.Data
// {
//     [Authorize(Roles = "Admin,Mod,BotMod")]
//     public class KenRQuoteService
//     {
//         private IDbContextFactory<ApplicationDbContext> DbFactory { get; set; }
//         public KenRQuoteService(IDbContextFactory<ApplicationDbContext> _dbFactory)
//         {
//             DbFactory = _dbFactory;
//         }
//         public async Task<KenRQuote[]> GetQuotesAsync()
//         {
//             using var db = DbFactory.CreateDbContext();
//             return await Task.FromResult(db.KenRQuotes.ToArray());
//         }

//         public async Task<int> DeleteQuoteAsync(KenRQuote quote)
//         {
//             using var db = DbFactory.CreateDbContext();
//             db.Remove(quote);
//             return await db.SaveChangesAsync();
//         }

//         public async Task<int> UpdateQuoteAsync(KenRQuote quote)
//         {
//             using var db = DbFactory.CreateDbContext();
//             db.Update(quote);
//             return await db.SaveChangesAsync();
//         }
//         public async Task<int> AddQuoteAsync(KenRQuote quote)
//         {
//             using var db = DbFactory.CreateDbContext();
//             db.Add(quote);
//             return await db.SaveChangesAsync();
//         }

//         //one time migration
//         // public async Task<int> AddQuoteAsync(KenRQuote quote)
//         // {
//         //     using var db = DbFactory.CreateDbContext();

//         //     var quotes = File.ReadAllLines("kenr_out_of_context_quotes.txt").ToList();

//         //     foreach (var item in quotes)
//         //     {
//         //         var q = new KenRQuote
//         //         {
//         //             Quote = item
//         //         };
//         //         db.Add(q);
//         //     }

//         //     return await db.SaveChangesAsync();
//         // }
//     }
// }
