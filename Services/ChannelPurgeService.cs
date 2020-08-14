using Discord;
using Discord.Commands;
using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Threading.Tasks;

namespace KenR_LeicaBot.Services
{
    public class ChannelPurgeService
    {
        public async Task PurgeAsync(SocketCommandContext context)
        {
            var msg = context.Message;
            var messages = await msg.Channel.GetMessagesAsync(100).FlattenAsync(); //defualt is 100

            // var messages = await msg.Channel.GetMessagesAsync(100);
            foreach (IUserMessage message in messages)
            {
                if (message.CreatedAt < DateTime.Now.AddDays(-6))
                {
                    //skip old messages!
                    continue;
                }

                if (message.Content.Contains("gotcha"))
                {
                    await message.DeleteAsync(); //rate limit exception if too many messages!!!

                }
            }
        }
    }
}
