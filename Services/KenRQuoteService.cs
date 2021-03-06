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

namespace KenR_LeicaBot.Services
{
    public class KenRQuoteService
    {
        private readonly AppConfig _config;
        private readonly DiscordSocketClient _discord;

        public KenRQuoteService(AppConfig config, DiscordSocketClient discord)
        {
            _config = config;
            _discord = discord;
        }
        public async Task PostRandomQuoteAsync(SocketCommandContext context)
        {
            var quote = GetRandomQuoteFromTwitter();
            await context.Channel.SendMessageAsync(quote);
        }

        private string GetRandomQuoteFromTwitter()
        {
            throw new NotImplementedException();
        }
    }
}
