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
    public class AdminCommandsModule : ModuleBase<SocketCommandContext>
    {
        // Dependency Injection will fill this value in for us
        public ChannelPurgeService ChannelPurgeService { get; set; }

        //dev test stuff
        [RequireUserPermission(GuildPermission.Administrator, Group = "Permission")]
        [Command("purge")]
        public async Task PurgeChannel()
        {
            await Task.Run(async () =>
             {
                 await ChannelPurgeService.PurgeAsync(this.Context);
             });
        }
    }
}
