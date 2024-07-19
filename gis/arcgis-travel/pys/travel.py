'''
This script should be run with ArcGIS Pro's native Python machine.
In windows, the running command would be like this:
c:\Progra~1\ArcGIS\Pro\bin\Python\scripts\propy.bat travel.py --from_id=5 --cutoffs0="0, 100, 500"
'''

'''
# expected result:
>> It's hard to go to somewhere within 0~100 from 종로타워
>> Within 500~1000, You can go to... 종로구청, 우정총국, 피마길, 광교 from 종로타워
>> Within 500~1000, You can go to... 세종대왕상, 광화문 from 종로타워
'''

from typing import List
from argparse import ArgumentParser
import arcpy # available via ArcGIS's Python API

# check accessibility
arcpy.CheckOutExtension("network")

parser = ArgumentParser()
# Starting points' OBJECTID
parser.add_argument("--from_id", default=None, type=str, required=True)
# Service Area cutoff ranges
parser.add_argument("--cutoffs0", default=None, type=str, required=True)

### arc gis settings (MANUAL)
# project workspace
arcpy.env.workspace = "../TravelProject/TravelProject.gdb"
# The Spatial Joined feature class
feature_class = "JoinResult"


def travel_from(
  from_id, cutoffs0: List[int]
) -> List[str]:
  
  cutoff0_places = {}

  for cutoff0 in cutoffs0:
    places = []
    # look up matching cases
    with arcpy.da.SearchCursor(feature_class, ["FacilityID", "FromBreak", "JOIN_OBJECTID"]) as c:
      for (fid, from_, pid) in c:
          if str(fid)==from_id and str(pid)!=from_id and from_==cutoff0 and pid is not None:
              places.append(int(pid))

    cutoff0_places[cutoff0] = places

  return cutoff0_places


def cutoff0_to_rng(cutoff0) -> str:
  if cutoff0==0:
    return "0~100"
  elif cutoff0==50:
    return "100~500"
  else:
    return "500~1000"
  

pid_to_name_dict = {
  1: "세종대왕상", 2: "광화문", 3: "종로구청", 4: "우정총국", 5: "종로타워",
  6: "피마길", 7: "광교", 8: "명동성당", 9: "서울광장", 10: "남대문", 11: "한국은행"
}

def pid_to_name(pid: int) -> str:
  return pid_to_name_dict.get(pid)


if __name__ == "__main__":
  
  args = parser.parse_args()
  from_id = args.from_id
  try:
    cutoffs0 = [int(e) for e in args.cutoffs0.split(", ")]
  except:
    raise Exception("cutoffs0 argument is wrong")

  cutoff0_places = travel_from(from_id, cutoffs0)

  from_id = int(from_id)

  for cutoff0, places in cutoff0_places.items():
    cutoff_ = cutoff0_to_rng(cutoff0)
    if len(places)==0:
      print(f"It's hard to go to somewhere within {cutoff_} from {pid_to_name(from_id)}")
    else:
      print(f"Within {cutoff_}, You can go to... {', '.join([pid_to_name(pid) for pid in places])} from {pid_to_name(from_id)}")