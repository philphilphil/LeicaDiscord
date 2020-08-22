using Discord;
using Discord.WebSocket;
using KenR_LeicaBot.Data;
using Microsoft.Extensions.Configuration;
using System;
using System.Collections.Generic;
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
        public Task ReactionAddedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            if (msg.Id != _config.Camera_Role_Message_Id) return Task.CompletedTask;

            foreach (var item in _config.Camera_Role_Maping)
            {
                if (react.Emote.Name == item.emoji)
                {
                  //  chan.SendMessageAsync(item.role);
                }
            }

            return Task.CompletedTask;
        }

        public Task ReactionRemovedEvent(Cacheable<IUserMessage, ulong> msg, ISocketMessageChannel chan, SocketReaction react)
        {
            if (msg.Id != _config.Camera_Role_Message_Id) return Task.CompletedTask;
            return Task.CompletedTask;
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
                }

                Thread.Sleep(1000);
            }

            return Task.CompletedTask;
        }
    }
}
