using Discord;
using Discord.WebSocket;
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
    public class RoleService
    {
        private readonly AppConfig _config;
        private readonly DiscordSocketClient _discord;

        public RoleService(AppConfig config, DiscordSocketClient discord)
        {
            _config = config;
            _discord = discord;
        }
        public async Task ReactionAddedEvent(Cacheable<IUserMessage, ulong> msg, Cacheable<IMessageChannel, ulong> chan, SocketReaction react)
        {
            if (msg.Id != _config.Camera_Role_Message_Id) return;
            await AddOrRemoveRoleOfUser(react, true);
            return;
        }

        public async Task ReactionRemovedEvent(Cacheable<IUserMessage, ulong> msg, Cacheable<IMessageChannel, ulong>  chan, SocketReaction react)
        {
            if (msg.Id != _config.Camera_Role_Message_Id) return;
            await AddOrRemoveRoleOfUser(react, false);
            return;
        }

        internal Task SetupRoleMessage()
        {
            var message = ((SocketTextChannel)_discord.GetChannel(_config.Camera_Role_Channel_Id)).GetMessageAsync(_config.Camera_Role_Message_Id).Result;

            foreach (var item in _config.Camera_Role_Maping)
            {
                var emoji = new Emoji(item.emoji);

                if (!message.Reactions.ContainsKey(emoji))
                {
                    message.AddReactionAsync(emoji);
                    Thread.Sleep(1000);
                }
            }

            return Task.CompletedTask;
        }
        private async Task AddOrRemoveRoleOfUser(SocketReaction react, bool add)
        {
            foreach (var item in _config.Camera_Role_Maping)
            {
                if (react.Emote.Name == item.emoji)
                {
                    var role = _discord.GetGuild(_config.Leica_Discord_Id).Roles.FirstOrDefault(x => x.Name == item.role);

                    if (role == null) return;

                    if (add)
                    {
                        await (react.User.Value as SocketGuildUser).AddRoleAsync(role);
                    }
                    else
                    {
                        await (react.User.Value as SocketGuildUser).RemoveRoleAsync(role);
                    }
                }
            }
        }
    }
}
