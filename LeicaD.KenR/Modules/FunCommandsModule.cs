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
    public class FunCommandsModule : ModuleBase<SocketCommandContext>
    {
        public KenRQuoteService KenRQuoteService { get; set; }

        [Command("50mm")]
        public Task Best50Async()
            => ReplyAsync("LEICA 50mm f/2 SUMMICRON-M - There is no better 50mm lens on Earth, or anywhere.");

        [Command("quote", true)]
        [Alias("q")]
        public async Task KenRQuoteAsync()
        {
            await KenRQuoteService.PostRandomQuoteAsync(this.Context);
        }

        // [Command("addquote")]
        // public async Task AddQuote(string quote)
        // {
        //     if (Context.User is SocketGuildUser user)
        //     {
        //         // Check if the user has the requried role
        //         if (user.Roles.Any(r => r.Name == "Bot Mod"))
        //         {
        //             await KenRQuoteService.AddQuoteToFile(this.Context, quote);
        //         }
        //     }
        // }
    }
}
