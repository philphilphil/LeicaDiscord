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
        [Command("purge")]
        public async Task PurgeChannel([Remainder] string channel = null)
        {
            if (string.IsNullOrEmpty(channel) || !ChannelExistsAndCanBePurged(channel))
            {
                await ReplyAsync("Please provide a valid Channelname");
                return;
            }

            await ChannelPurgeService.PurgeAsync(this.Context, channel);
        }

        // Get info on a user, or the user who invoked the command if one is not specified
        [Command("userinfo")]
        public async Task UserInfoAsync(IUser user = null)
        {
            user = user ?? Context.User;

            await ReplyAsync(user.ToString());
        }

        private bool ChannelExistsAndCanBePurged(string channel)
        {
            //security feature to not accidentally purge the wrong channels... 
            List<string> purgableChannels = new List<string>(); //TODO: add this list to config/database
            purgableChannels.Add("botdebug");
            purgableChannels.Add("gallery");
            purgableChannels.Add("your-best-photos");

            if (!this.Context.Guild.TextChannels.Where(x => x.Name == channel).Any()) return false;

            if (!purgableChannels.Contains(channel)) return false;


            return true;
        }
    }
}
