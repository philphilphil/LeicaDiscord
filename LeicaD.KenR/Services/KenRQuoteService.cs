using Discord;
using Discord.WebSocket;
using Discord.Commands;
using KenR_LeicaBot.Data;
using Microsoft.Extensions.Configuration;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.IO;
using LeicaD.Web.Data;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Sqlite;

namespace KenR_LeicaBot.Services
{
    public class KenRQuoteService
    {
        private readonly AppConfig _config;
        private readonly DiscordSocketClient _discord;

        private const string QUOTE_DB_PATH = "Databases/kenr_out_of_context_quotes.txt";

        public KenRQuoteService(AppConfig config, DiscordSocketClient discord)
        {
            _config = config;
            _discord = discord;
        }
        public async Task PostRandomQuoteAsync(SocketCommandContext context)
        {
            var quote = await GetRandomQuoteFromDbAsync();
            await context.Channel.SendMessageAsync(quote);
        }

        private async Task<string> GetRandomQuoteFromDbAsync()
        {
            var options = new DbContextOptionsBuilder<ApplicationDbContext>().UseSqlite(_config.ConnectionString).Options;

            using (var db = new ApplicationDbContext(options))
            {
                var quotes = db.KenRQuotes.AsQueryable().Where(x => x.LastPosted == null).ToList();

                if (quotes.Count() == 0)
                { //all quotes posted, reset
                    var allQuotes = db.KenRQuotes.ToList();
                    allQuotes.ForEach(x => x.LastPosted = null);
                    await db.SaveChangesAsync();
                    quotes = db.KenRQuotes.AsQueryable().Where(x => x.LastPosted == null).ToList();
                }

                Random rnd = new Random();
                int chosenQuote = rnd.Next(0, quotes.Count());
                var quote = quotes[chosenQuote];
                quote.LastPosted = DateTime.Now;
                await db.SaveChangesAsync();
                return quote.Quote;
            }
        }

        // public async Task AddQuoteToFile(SocketCommandContext context, string quote)
        // {
        //     File.AppendAllLines(QUOTE_DB_PATH, new[] { quote });
        //     await context.Message.AddReactionAsync(new Emoji("\U00002705"));
        //     //await context.Channel.SendMessageAsync("✅");
        // }
    }
}
