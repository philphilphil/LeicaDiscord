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
            var quote = GetRandomQuoteFromFile();
            await context.Channel.SendMessageAsync(quote);
        }

        private string GetRandomQuoteFromFile()
        {
            var quotes = File.ReadAllLines(QUOTE_DB_PATH).Where(x => !string.IsNullOrWhiteSpace(x)).ToList();
            Random rnd = new Random();
            int chosenQuote = rnd.Next(0, quotes.Count);
            return quotes[chosenQuote];
        }

        public async Task AddQuoteToFile(SocketCommandContext context, string quote)
        {
            File.AppendAllLines(QUOTE_DB_PATH, new[] { quote });
            await context.Message.AddReactionAsync(new Emoji("\U00002705"));
            //await context.Channel.SendMessageAsync("✅");
        }
    }
}
