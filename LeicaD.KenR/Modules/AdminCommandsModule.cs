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
    public class AdminCommandsModule : ModuleBase<SocketCommandContext>
    {
        public ChannelPurgeService ChannelPurgeService { get; set; }

        [RequireUserPermission(GuildPermission.Administrator, Group = "Permission")]
        [Command("purge", RunMode = RunMode.Async)]
        public async Task PurgeChannel()
        {
            await Task.Run(async () =>
             {
                 await ChannelPurgeService.PurgeAsync(this.Context);
             });
        }
    }
}
