#include <linux/list.h>
#include <scsi/scsi_common.h>

#define CDROM_STR_SIZE 1000

struct cdrom_sysctl_settings {						//AL: need this
	char	info[CDROM_STR_SIZE];	/* general info */
	int	autoclose;		/* close tray upon mount, etc */
	int	autoeject;		/* eject on umount */
	int	debug;			/* turn on debugging messages */
	int	lock;			/* lock the door on device open */
	int	check;			/* check media type */
};

int cdrom_sysctl_info(struct ctl_table *ctl, int write,
                           void *buffer, size_t *lenp, loff_t *ppos);

int cdrom_sysctl_handler(struct ctl_table *ctl, int write,
				void *buffer, size_t *lenp, loff_t *ppos);