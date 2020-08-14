using System.IO;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using KenR_LeicaBot.Services;

namespace KenR_LeicaBot.Modules
{
    // Modules must be public and inherit from an IModuleBase
    public class PublicModule : ModuleBase<SocketCommandContext>
    {
        // Dependency Injection will fill this value in for us
        public ChannelPurgeService ChannelPurgeService { get; set; }

        [Command("50mm")]
        [Alias("hi")]
        public Task PingAsync()
            => ReplyAsync("LEICA 50mm f/2 SUMMICRON-M - There is no better 50mm lens on Earth, or anywhere.");

        //dev test stuff
        [RequireUserPermission(GuildPermission.Administrator, Group = "Permission")]
        [RequireOwner(Group = "Permission")]
        [Command("test")]
        public async Task DevelopmentTest(IUser user = null)
        {
            await ChannelPurgeService.PurgeAsync(this.Context);
        }

        // Get info on a user, or the user who invoked the command if one is not specified
        [Command("userinfo")]
        public async Task UserInfoAsync(IUser user = null)
        {
            user = user ?? Context.User;

            await ReplyAsync(user.ToString());
        }

 
    }
}
