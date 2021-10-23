#include <errno.h>
#include <linux/ipmi.h>
#include <sys/ioctl.h>

extern int si_cmd(int fd, unsigned char netfn, unsigned char cmd, unsigned char *data, unsigned short data_len) {
	struct ipmi_msg msg;
	msg.netfn = netfn;
	msg.cmd = cmd;
	msg.data = data;
	msg.data_len = data_len;

	struct ipmi_system_interface_addr addr;
	addr.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
	addr.channel = IPMI_BMC_CHANNEL;

	struct ipmi_req req;
	req.addr = (unsigned char*)&addr;
	req.addr_len = sizeof(addr);
	req.msgid = 0;
	req.msg = msg;

	int result = ioctl(fd, IPMICTL_SEND_COMMAND, &req);
	if (result != 0)
		return errno;

	// successfully sent command; completion code returned in data[0]
	return 0;
}
