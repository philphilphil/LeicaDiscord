using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using KenR_LeicaBot.Services;

namespace KenR_LeicaBot.Modules
{
    // Modules must be public and inherit from an IModuleBase
    public class FunCommandsModule : ModuleBase<SocketCommandContext>
    {
        // Dependency Injection will fill this value in for us
        public KenRQuoteService KenRQuoteService { get; set; }

        [Command("50mm")]
        public Task PingAsync()
            => ReplyAsync("LEICA 50mm f/2 SUMMICRON-M - There is no better 50mm lens on Earth, or anywhere.");

        [Command("quote")]
        [Alias("q")]
        public async Task KenRQuote()
        {
            await Task.Run(async () =>
             {
                 await KenRQuoteService.PostRandomQuoteAsync(this.Context);
             });
        }

        [Command("addquote")]
        public async Task AddQuote(string quote)
        {
            if (Context.User is SocketGuildUser user)
            {
                // Check if the user has the requried role
                if (user.Roles.Any(r => r.Name == "Bot Mod"))
                {
                    await KenRQuoteService.AddQuoteToFile(this.Context, quote);
                }
            }
        }
    }
}
